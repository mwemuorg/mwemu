use sha2::{Digest, Sha256};

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{crypto_state, heap, memory, strings};
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::CRYPTPROTECT_UI_FORBIDDEN;
use crate::windows::constants::errors::*;

const GUEST_DPAPI: &[u8] = b"mwemu-dpapi\x01";
const CHECKSUM_BYTES: usize = 32;

fn blob(emu: &Emu, descriptor: u64) -> Result<Vec<u8>, u64> {
    let len = emu
        .maps
        .read_dword(descriptor)
        .ok_or(ERROR_INVALID_PARAMETER)?;
    let pointer = descriptor
        .checked_add(memory::pointer_size(emu))
        .and_then(|addr| memory::read_pointer(emu, addr))
        .ok_or(ERROR_INVALID_PARAMETER)?;
    if u64::from(len) > emu.maps.max_alloc_size {
        return Err(ERROR_NOT_ENOUGH_MEMORY);
    }
    if len == 0 {
        return Ok(Vec::new());
    }
    memory::read_guest_bytes(emu, pointer, len as usize).ok_or(ERROR_NOACCESS)
}

fn checksum(bytes: &[u8], entropy: &[u8]) -> [u8; CHECKSUM_BYTES] {
    let mut hash = Sha256::new();
    hash.update((entropy.len() as u64).to_le_bytes());
    hash.update(entropy);
    hash.update(bytes);
    hash.finalize().into()
}

fn protect(data: &[u8], description: &[u8], entropy: &[u8]) -> Vec<u8> {
    let cipher = crypto_state::dpapi_protect(data);
    let mut result = GUEST_DPAPI.to_vec();
    result.extend((description.len() as u32).to_le_bytes());
    result.extend(description);
    result.extend(cipher);
    result.extend(checksum(&result, entropy));
    result
}

fn unprotect<'a>(data: &'a [u8], entropy: &[u8]) -> Option<(Vec<u8>, &'a [u8])> {
    let (body, tag) = data.split_at_checked(data.len().checked_sub(CHECKSUM_BYTES)?)?;
    if checksum(body, entropy).as_slice() != tag {
        return None;
    }
    let (size, body) = body.strip_prefix(GUEST_DPAPI)?.split_first_chunk::<4>()?;
    let (description, protected) = body.split_at_checked(u32::from_le_bytes(*size) as usize)?;
    let (len, cipher) = protected.split_first_chunk::<4>()?;
    let len = u32::from_le_bytes(*len) as usize;
    if description.len() % 2 != 0 || cipher.len() != len.div_ceil(16).checked_mul(16)? {
        return None;
    }
    Some((
        if len == 0 {
            Vec::new()
        } else {
            crypto_state::dpapi_unprotect(protected)
        },
        description,
    ))
}

fn protect_data(emu: &mut Emu, api: &str, decrypt: bool) {
    let call = WinApi(Stdcall);
    let [input, description, entropy, reserved, prompt, flags, output] =
        api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "crypt32!{api} input={input:#x} description={description:#x} entropy={entropy:#x} flags={flags:#x} output={output:#x}"
    );
    let result = (|| {
        let pointer_size = memory::pointer_size(emu);
        if reserved != 0
            || input == 0
            || output == 0
            || !emu.maps.validate_write_range(output, pointer_size * 2)
        {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if prompt != 0 || flags as u32 & !CRYPTPROTECT_UI_FORBIDDEN != 0 {
            return Err(ERROR_NOT_SUPPORTED);
        }
        if decrypt && description != 0 && !memory::writable_pointer(emu, description) {
            return Err(ERROR_NOACCESS);
        }
        let bytes = blob(emu, input)?;
        let entropy = if entropy == 0 {
            Vec::new()
        } else {
            blob(emu, entropy)?
        };
        let (transformed, description_bytes) = if decrypt {
            unprotect(&bytes, &entropy).ok_or(ERROR_INVALID_DATA)?
        } else {
            let description = if description == 0 {
                Vec::new()
            } else {
                let text =
                    strings::read_text_checked(emu, description, true).ok_or(ERROR_NOACCESS)?;
                strings::encode_text(&text, true)
            };
            (protect(&bytes, &description, &entropy), &[][..])
        };
        let data =
            heap::heap_allocate(emu, 0, transformed.len() as u64).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
        let mut string = 0;
        if decrypt && description != 0 {
            let mut text = description_bytes.to_vec();
            text.extend([0, 0]);
            let Some(pointer) = heap::heap_allocate(emu, 0, text.len() as u64) else {
                heap::heap_free(emu, 0, data);
                return Err(ERROR_NOT_ENOUGH_MEMORY);
            };
            string = pointer;
            emu.maps.write_bytes(string, &text);
        }
        emu.maps.write_bytes(data, &transformed);
        emu.maps.write_dword(output, transformed.len() as u32);
        memory::write_pointer(emu, output + pointer_size, data);
        if decrypt && description != 0 {
            memory::write_pointer(emu, description, string);
        }
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 7);
}
pub fn CryptProtectData(emu: &mut Emu) {
    protect_data(emu, "CryptProtectData", false);
}
pub fn CryptUnprotectData(emu: &mut Emu) {
    protect_data(emu, "CryptUnprotectData", true);
}
