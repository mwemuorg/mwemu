use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{memory, trace::input_string};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::crypto::*;
use crate::windows::constants::errors::*;

const MAX_CRYPTO_BUFFER: usize = 16 << 20;
pub(super) const BASE_PROVIDER_NAME: &str = "Microsoft Base Cryptographic Provider v1.0";
pub(super) const PROVIDER_URI: &str = "cryptctx://";

pub(super) fn is_csp_key(emu: &Emu, handle: u64) -> bool {
    let uri = emu.crypto.name(handle);
    uri.starts_with("cryptkey:") || uri.starts_with("derivedkey:")
}

pub(super) fn is_csp_hash(emu: &Emu, handle: u64) -> bool {
    emu.crypto.name(handle).starts_with("crypthash:")
}

fn result(emu: &mut Emu, operation: Result<(), u64>, arguments: usize) {
    let call = WinApi(Stdcall);
    if let Err(error) = operation {
        emu.set_last_error(error);
    }
    call.set_return(emu, operation.is_ok().into());
    call.cleanup_args(emu, arguments);
}

fn acquire(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [output, container, provider_name, provider_type, flags] = api_try!(emu, call.args(emu));
    let encoding = Encoding::from_wide(wide);
    log_red!(
        emu,
        "advapi32!{api} provider_out={output:#x} container={} provider_name={} provider_type={provider_type} flags={flags:#x}",
        input_string(emu, container, encoding),
        input_string(emu, provider_name, encoding)
    );
    if output == 0 || !memory::writable_pointer(emu, output) {
        result(emu, Err(ERROR_NOACCESS), 5);
        return;
    }
    let known_provider = provider_name == 0
        || match encoding.read(emu, provider_name) {
            Ok(name) => String::from_utf16_lossy(&name).eq_ignore_ascii_case(BASE_PROVIDER_NAME),
            Err(error) => {
                result(emu, Err(error), 5);
                return;
            }
        };
    if provider_type as u32 != PROV_RSA_FULL
        || container != 0
        || !known_provider
        || (flags as u32) & !(CRYPT_VERIFYCONTEXT | CRYPT_SILENT) != 0
    {
        policy::unsupported(emu, "advapi32", api, emu.pc());
        return;
    }
    let provider = emu.crypto.open(PROVIDER_URI);
    memory::write_pointer(emu, output, provider);
    result(emu, Ok(()), 5);
}

pub fn CryptAcquireContextA(emu: &mut Emu) {
    acquire(emu, "CryptAcquireContextA", false);
}

pub fn CryptAcquireContextW(emu: &mut Emu) {
    acquire(emu, "CryptAcquireContextW", true);
}

pub fn CryptCreateHash(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, algorithm, key, flags, output] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "advapi32!CryptCreateHash provider={provider:#x} algorithm={algorithm:#x} key={key:#x} flags={flags:#x} hash_out={output:#x}"
    );
    let operation = if output == 0 || !memory::writable_pointer(emu, output) {
        Err(ERROR_NOACCESS)
    } else if emu.crypto.name(provider) != PROVIDER_URI {
        Err(ERROR_INVALID_HANDLE)
    } else if flags as u32 != 0
        || key != 0
        || !matches!(algorithm as u32, CALG_MD5 | CALG_SHA1 | CALG_SHA_256)
    {
        policy::unsupported(emu, "advapi32", "CryptCreateHash", emu.pc());
        return;
    } else {
        let handle = emu.crypto.open("crypthash://");
        if emu.crypto.create_hash(handle, algorithm as u32)
            && memory::write_pointer(emu, output, handle)
        {
            Ok(())
        } else {
            emu.crypto.destroy_hash(handle);
            emu.crypto.close(handle);
            Err(ERROR_INVALID_PARAMETER)
        }
    };
    result(emu, operation, 5);
}

pub fn CryptHashData(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [hash, data, length, flags] = api_try!(emu, call.args(emu));
    let length = length as u32 as usize;
    log_red!(
        emu,
        "advapi32!CryptHashData hash={hash:#x} data={data:#x} length={length} flags={flags:#x}"
    );
    if !is_csp_hash(emu, hash) {
        result(emu, Err(NTE_BAD_HASH), 4);
        return;
    }
    if flags as u32 != 0 || length > MAX_CRYPTO_BUFFER {
        policy::unsupported(emu, "advapi32", "CryptHashData", emu.pc());
        return;
    }
    let operation = if length == 0 {
        Some(Vec::new())
    } else {
        memory::read_guest_bytes(emu, data, length)
    }
    .ok_or(ERROR_NOACCESS)
    .and_then(|bytes| {
        if emu.crypto.hash_data(hash, &bytes) {
            Ok(())
        } else {
            Err(NTE_BAD_HASH_STATE)
        }
    });
    result(emu, operation, 4);
}

fn transform(emu: &mut Emu, api: &str, encrypt: bool) {
    let call = WinApi(Stdcall);
    let [key, hash, final_block, flags, data, length_out] = api_try!(emu, call.args(emu));
    let capacity = if encrypt {
        api_try!(emu, call.arg_u32(emu, 6))
    } else {
        u32::MAX
    };
    let nargs = 6 + usize::from(encrypt);
    log_red!(
        emu,
        "advapi32!{api} key={key:#x} hash={hash:#x} final_block={final_block} flags={flags:#x} data={data:#x} data_length={length_out:#x}"
    );
    if !is_csp_key(emu, key) {
        result(emu, Err(NTE_BAD_KEY), nargs);
        return;
    }
    if hash != 0 && !is_csp_hash(emu, hash) {
        result(emu, Err(NTE_BAD_HASH), nargs);
        return;
    }
    let Some(length) = emu.maps.read_dword(length_out).filter(|_| {
        length_out != 0
            && emu
                .maps
                .validate_write_range(length_out, size_of::<u32>() as u64)
    }) else {
        result(emu, Err(ERROR_NOACCESS), nargs);
        return;
    };
    let algorithm = emu
        .crypto
        .get_key_param(key, KP_ALGID)
        .and_then(|b| b.try_into().ok())
        .map(u32::from_le_bytes);
    let Some(algorithm) = algorithm else {
        result(emu, Err(NTE_BAD_KEY), nargs);
        return;
    };
    // The stream model resets after a final call. Chaining, padding and hash updates need a complete CSP model.
    if algorithm != CALG_RC4
        || final_block as u32 == 0
        || flags as u32 != 0
        || hash != 0
        || length as usize > MAX_CRYPTO_BUFFER
    {
        policy::unsupported(emu, "advapi32", api, emu.pc());
        return;
    }
    let operation = if encrypt && data == 0 {
        Ok(())
    } else if length > capacity {
        Err(ERROR_MORE_DATA)
    } else if data == 0 || !emu.maps.validate_write_range(data, length.into()) {
        Err(ERROR_NOACCESS)
    } else {
        memory::read_guest_bytes(emu, data, length as usize)
            .ok_or(ERROR_NOACCESS)
            .and_then(|mut bytes| {
                let success = if encrypt {
                    emu.crypto.encrypt_data(key, &mut bytes)
                } else {
                    emu.crypto.decrypt_data(key, &mut bytes)
                };
                if !success {
                    return Err(NTE_BAD_KEY);
                }
                if !bytes.is_empty() && !emu.maps.write_bytes(data, &bytes) {
                    return Err(ERROR_NOACCESS);
                }
                Ok(())
            })
    };
    result(emu, operation, nargs);
}

pub fn CryptEncrypt(emu: &mut Emu) {
    transform(emu, "CryptEncrypt", true);
}

pub fn CryptDecrypt(emu: &mut Emu) {
    transform(emu, "CryptDecrypt", false);
}

pub fn CryptReleaseContext(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, flags] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "advapi32!CryptReleaseContext provider={provider:#x} flags={flags:#x}"
    );
    let operation = if flags as u32 != 0 {
        Err(ERROR_INVALID_PARAMETER)
    } else if emu.crypto.name(provider) == PROVIDER_URI && emu.crypto.close(provider) {
        Ok(())
    } else {
        Err(ERROR_INVALID_HANDLE)
    };
    result(emu, operation, 2);
}
