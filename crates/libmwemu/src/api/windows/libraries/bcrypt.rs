mod dispatch;
pub(crate) use dispatch::dispatch;
pub mod exports;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::{crypto_state, memory, strings};
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::crypto::*;
use crate::windows::constants::errors::*;

pub fn ProcessPrng(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [buffer, size] = api_try!(emu, call.args(emu));
    let ok = emu.fill_random(buffer, size);
    log_red!(
        emu,
        "bcryptprimitives!ProcessPrng buffer={buffer:#x} size={size} success={ok}"
    );
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 2);
}

pub fn BCryptGenRandom(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let provider = api_try!(emu, call.arg(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg_u32(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let system = flags & BCRYPT_USE_SYSTEM_PREFERRED_RNG != 0;
    let status = if flags & !(BCRYPT_USE_SYSTEM_PREFERRED_RNG | BCRYPT_RNG_USE_ENTROPY_IN_BUFFER)
        != 0
        || (system && provider != 0)
    {
        STATUS_INVALID_PARAMETER
    } else if !system
        && provider != BCRYPT_RNG_ALG_HANDLE
        && !helper::handler_get_uri(provider).eq_ignore_ascii_case("bcrypt://RNG")
    {
        STATUS_INVALID_HANDLE
    } else if buffer == 0 || !emu.fill_random(buffer, size.into()) {
        STATUS_INVALID_PARAMETER
    } else {
        STATUS_SUCCESS
    };
    log_red!(
        emu,
        "bcrypt!BCryptGenRandom provider={provider:#x} buffer={buffer:#x} size={size} flags={flags:#x} status={status:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 4);
}

fn hash_algorithm(provider: u64) -> Result<(&'static str, u32), u64> {
    match provider {
        BCRYPT_MD5_ALG_HANDLE => return Ok(("MD5", CALG_MD5)),
        BCRYPT_SHA1_ALG_HANDLE => return Ok(("SHA1", CALG_SHA1)),
        BCRYPT_SHA256_ALG_HANDLE => return Ok(("SHA256", CALG_SHA_256)),
        _ => {}
    }
    let uri = helper::handler_get_uri(provider);
    if uri.starts_with("bcrypt-hmac://") {
        return Err(STATUS_NOT_SUPPORTED);
    }
    match uri.strip_prefix("bcrypt://") {
        Some("MD5") => Ok(("MD5", CALG_MD5)),
        Some("SHA1") => Ok(("SHA1", CALG_SHA1)),
        Some("SHA256") => Ok(("SHA256", CALG_SHA_256)),
        Some(_) => Err(STATUS_NOT_SUPPORTED),
        None => Err(STATUS_INVALID_HANDLE),
    }
}

fn create_hash(provider: u64) -> Result<u64, u64> {
    let (name, alg_id) = hash_algorithm(provider)?;
    let handle = helper::handler_create(&format!("bcrypthash://{name}"));
    crypto_state::create_hash(handle, alg_id);
    Ok(handle)
}

fn release_hash(handle: u64) -> bool {
    if !helper::handler_get_uri(handle).starts_with("bcrypthash://")
        || !crypto_state::destroy_hash(handle)
    {
        return false;
    }
    helper::handler_close(handle);
    true
}

fn update_hash(emu: &Emu, handle: u64, input: u64, size: usize) -> Result<(), u64> {
    if !helper::handler_get_uri(handle).starts_with("bcrypthash://")
        || !crypto_state::hash_data(handle, &[])
    {
        return Err(STATUS_INVALID_HANDLE);
    }
    if size != 0 && (input == 0 || !emu.maps.can_access_range(input, size, Permission::READ)) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let mut consumed = 0;
    while consumed < size {
        let address = input + consumed as u64;
        let map = emu
            .maps
            .get_mem_by_addr(address)
            .ok_or(STATUS_INVALID_PARAMETER)?;
        let length = (map.get_bottom() - address).min((size - consumed) as u64) as usize;
        let data = map.read_bytes(address, length);
        if data.len() != length || !crypto_state::hash_data(handle, &data) {
            return Err(STATUS_INVALID_PARAMETER);
        }
        consumed += length;
    }
    Ok(())
}

fn finish_hash(emu: &mut Emu, handle: u64, output: u64, size: u32) -> Result<(), u64> {
    if !helper::handler_get_uri(handle).starts_with("bcrypthash://")
        || !crypto_state::hash_data(handle, &[])
    {
        return Err(STATUS_INVALID_HANDLE);
    }
    let expected =
        crypto_state::get_hash_param(handle, HP_HASHSIZE).ok_or(STATUS_INVALID_HANDLE)?;
    if output == 0
        || size.to_le_bytes().as_slice() != expected
        || !emu.maps.validate_write_range(output, u64::from(size))
    {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let digest = crypto_state::get_hash_param(handle, HP_HASHVAL).ok_or(STATUS_INVALID_HANDLE)?;
    if emu.maps.write_bytes(output, &digest) {
        Ok(())
    } else {
        Err(STATUS_INVALID_PARAMETER)
    }
}

pub fn BCryptCreateHash(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [
        provider,
        output,
        object,
        object_size,
        secret,
        secret_size,
        flags,
    ] = api_try!(emu, call.args(emu));
    let result =
        if !memory::writable_pointer(emu, output) || (object == 0 && object_size as u32 != 0) {
            Err(STATUS_INVALID_PARAMETER)
        } else if flags as u32 != 0 || secret != 0 || secret_size as u32 != 0 {
            Err(STATUS_NOT_SUPPORTED)
        } else {
            create_hash(provider).and_then(|handle| {
                if memory::write_pointer(emu, output, handle) {
                    Ok(())
                } else {
                    release_hash(handle);
                    Err(STATUS_INVALID_PARAMETER)
                }
            })
        };
    log_red!(
        emu,
        "bcrypt!BCryptCreateHash provider={provider:#x} output={output:#x} flags={flags:#x}"
    );
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 7);
}

pub fn BCryptHashData(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [handle, input, size, flags] = api_try!(emu, call.args(emu));
    let result = if flags as u32 == 0 {
        update_hash(emu, handle, input, size as u32 as usize)
    } else {
        Err(STATUS_INVALID_PARAMETER)
    };
    log_red!(
        emu,
        "bcrypt!BCryptHashData hash={handle:#x} input={input:#x} size={size} flags={flags:#x}"
    );
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 4);
}

pub fn BCryptFinishHash(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [handle, output, size, flags] = api_try!(emu, call.args(emu));
    let result = if flags as u32 == 0 {
        finish_hash(emu, handle, output, size as u32)
    } else {
        Err(STATUS_INVALID_PARAMETER)
    };
    log_red!(
        emu,
        "bcrypt!BCryptFinishHash hash={handle:#x} output={output:#x} size={size} flags={flags:#x}"
    );
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 4);
}

pub fn BCryptHash(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, secret, secret_size, input, size, output, capacity] =
        api_try!(emu, call.args(emu));
    let result = if secret != 0 || secret_size as u32 != 0 {
        Err(STATUS_NOT_SUPPORTED)
    } else {
        create_hash(provider).and_then(|handle| {
            let result = update_hash(emu, handle, input, size as u32 as usize)
                .and_then(|()| finish_hash(emu, handle, output, capacity as u32));
            release_hash(handle);
            result
        })
    };
    log_red!(
        emu,
        "bcrypt!BCryptHash provider={provider:#x} input_size={size} output_size={capacity}"
    );
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 7);
}

pub fn BCryptDestroyHash(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hash = api_try!(emu, call.arg(emu, 0));
    let status = if release_hash(hash) {
        STATUS_SUCCESS
    } else {
        STATUS_INVALID_HANDLE
    };
    log_red!(emu, "bcrypt!BCryptDestroyHash hash={hash:#x}");
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}

pub fn BCryptOpenAlgorithmProvider(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [output, algorithm, implementation, flags] = api_try!(emu, call.args(emu));
    let name = strings::read_text_checked(emu, algorithm, true);
    let provider = if implementation == 0 {
        None
    } else {
        strings::read_text_checked(emu, implementation, true)
    };
    let result = if !memory::writable_pointer(emu, output)
        || name.is_none()
        || (implementation != 0 && provider.is_none())
    {
        Err(STATUS_INVALID_PARAMETER)
    } else if flags as u32 & !BCRYPT_ALG_HANDLE_HMAC_FLAG != 0
        || provider
            .as_deref()
            .is_some_and(|p| !p.eq_ignore_ascii_case("Microsoft Primitive Provider"))
    {
        Err(STATUS_NOT_SUPPORTED)
    } else {
        let name = name.as_deref().unwrap_or_default().to_ascii_uppercase();
        match name.as_str() {
            "MD5" | "SHA1" | "SHA256" | "AES" | "3DES" | "RC4" | "RNG" | "RSA" | "DSA"
            | "ECDSA_P256" | "ECDH_P256" => {
                let prefix = if flags as u32 & BCRYPT_ALG_HANDLE_HMAC_FLAG != 0 {
                    "bcrypt-hmac"
                } else {
                    "bcrypt"
                };
                let handle = helper::handler_create(&format!("{prefix}://{name}"));
                if memory::write_pointer(emu, output, handle) {
                    Ok(())
                } else {
                    helper::handler_close(handle);
                    Err(STATUS_INVALID_PARAMETER)
                }
            }
            _ => Err(STATUS_NOT_SUPPORTED),
        }
    };
    log_red!(
        emu,
        "bcrypt!BCryptOpenAlgorithmProvider algorithm={name:?} implementation={provider:?} flags={flags:#x}"
    );
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 4);
}

pub fn BCryptCloseAlgorithmProvider(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, flags] = api_try!(emu, call.args(emu));
    let uri = helper::handler_get_uri(provider);
    let status = if flags as u32 != 0 {
        STATUS_INVALID_PARAMETER
    } else if uri.starts_with("bcrypt://") || uri.starts_with("bcrypt-hmac://") {
        helper::handler_close(provider);
        STATUS_SUCCESS
    } else {
        STATUS_INVALID_HANDLE
    };
    log_red!(
        emu,
        "bcrypt!BCryptCloseAlgorithmProvider provider={provider:#x} flags={flags:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 2);
}

fn cng_input(emu: &Emu, address: u64, length: u32) -> Result<Vec<u8>, u64> {
    if u64::from(length) > emu.maps.max_alloc_size {
        return Err(STATUS_NOT_SUPPORTED);
    }
    if length == 0 {
        return Ok(Vec::new());
    }
    memory::read_guest_bytes(emu, address, length as usize).ok_or(STATUS_INVALID_PARAMETER)
}

pub fn BCryptDeriveKeyPBKDF2(emu: &mut Emu) {
    const MAX_HMAC_OPERATIONS: u64 = 1_000_000;
    let call = WinApi(Stdcall);
    let [provider, password, password_size, salt, salt_size] = api_try!(emu, call.args(emu));
    let iterations = api_try!(emu, call.arg_u64(emu, 5));
    let next = if emu.cfg.arch.is_64bits() { 6 } else { 7 };
    let output = api_try!(emu, call.arg(emu, next));
    let size = api_try!(emu, call.arg_u32(emu, next + 1));
    let flags = api_try!(emu, call.arg_u32(emu, next + 2));
    log_red!(
        emu,
        "bcrypt!BCryptDeriveKeyPBKDF2 provider={provider:#x} password_size={password_size} salt_size={salt_size} iterations={iterations} output_size={size}"
    );
    let result = (|| {
        let uri = helper::handler_get_uri(provider);
        let (alg_id, block) = match (provider, uri.as_str()) {
            (BCRYPT_HMAC_SHA1_ALG_HANDLE, _) | (_, "bcrypt-hmac://SHA1") => (CALG_SHA1, 20),
            (BCRYPT_HMAC_SHA256_ALG_HANDLE, _) | (_, "bcrypt-hmac://SHA256") => (CALG_SHA_256, 32),
            _ => return Err(STATUS_INVALID_HANDLE),
        };
        if flags != 0
            || size == 0
            || iterations == 0
            || output == 0
            || !emu.maps.validate_write_range(output, u64::from(size))
        {
            return Err(STATUS_INVALID_PARAMETER);
        }
        if iterations.saturating_mul(u64::from(size).div_ceil(block)) > MAX_HMAC_OPERATIONS
            || u64::from(size) > emu.maps.max_alloc_size
        {
            return Err(STATUS_NOT_SUPPORTED);
        }
        let password = cng_input(emu, password, password_size as u32)?;
        let salt = cng_input(emu, salt, salt_size as u32)?;
        let mut key = vec![0; size as usize];
        crypto_state::pbkdf2_derive(alg_id, &password, &salt, iterations as u32, &mut key);
        if emu.maps.write_bytes(output, &key) {
            Ok(())
        } else {
            Err(STATUS_INVALID_PARAMETER)
        }
    })();
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 10);
}

pub fn BCryptGenerateSymmetricKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [
        provider,
        output,
        object,
        object_size,
        secret,
        secret_size,
        flags,
    ] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "bcrypt!BCryptGenerateSymmetricKey provider={provider:#x} secret_size={secret_size} output={output:#x}"
    );
    let result = (|| {
        let length = secret_size as u32;
        if flags as u32 != 0
            || !memory::writable_pointer(emu, output)
            || (object == 0 && object_size as u32 != 0)
        {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let uri = helper::handler_get_uri(provider);
        let (algorithm, key_type) = match (provider, uri.as_str(), length) {
            (BCRYPT_AES_CBC_ALG_HANDLE, _, 16) | (_, "bcrypt://AES", 16) => {
                (CALG_AES_128, "aes-cbc")
            }
            (BCRYPT_AES_CBC_ALG_HANDLE, _, 32) | (_, "bcrypt://AES", 32) => {
                (CALG_AES_256, "aes-cbc")
            }
            (BCRYPT_3DES_CBC_ALG_HANDLE, _, 24) | (_, "bcrypt://3DES", 24) => {
                (CALG_3DES, "3des-cbc")
            }
            (BCRYPT_RC4_ALG_HANDLE, _, 1..=256) | (_, "bcrypt://RC4", 1..=256) => (CALG_RC4, "rc4"),
            (_, uri, _) if uri.starts_with("bcrypt://") => return Err(STATUS_NOT_SUPPORTED),
            _ => return Err(STATUS_INVALID_HANDLE),
        };
        let bytes = cng_input(emu, secret, length)?;
        let key = helper::handler_create(&format!("bcryptkey://{key_type}"));
        crypto_state::create_key(key, algorithm, length * 8);
        crypto_state::set_key_data(key, &bytes);
        if memory::write_pointer(emu, output, key) {
            Ok(())
        } else {
            crypto_state::destroy_key(key);
            helper::handler_close(key);
            Err(STATUS_INVALID_PARAMETER)
        }
    })();
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 7);
}

pub fn BCryptDestroyKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let key = api_try!(emu, call.arg(emu, 0));
    let ok =
        helper::handler_get_uri(key).starts_with("bcryptkey://") && crypto_state::destroy_key(key);
    if ok {
        helper::handler_close(key);
    }
    log_red!(emu, "bcrypt!BCryptDestroyKey key={key:#x}");
    call.set_return(
        emu,
        if ok {
            STATUS_SUCCESS
        } else {
            STATUS_INVALID_HANDLE
        },
    );
    call.cleanup_args(emu, 1);
}

fn cipher(emu: &mut Emu, name: &str, decrypt: bool) {
    let call = WinApi(Stdcall);
    let [
        key,
        input,
        input_len,
        padding,
        iv,
        iv_len,
        output,
        capacity,
        written,
        flags,
    ] = api_try!(emu, call.args(emu));
    let length = input_len as u32 as usize;
    let flags = flags as u32;
    log_red!(
        emu,
        "bcrypt!{name} key={key:#x} input_size={length} iv_size={iv_len} output_size={capacity} flags={flags:#x}"
    );
    let result = (|| {
        let block: usize = match helper::handler_get_uri(key).as_str() {
            "bcryptkey://rc4" => 1,
            "bcryptkey://aes-cbc" => 16,
            "bcryptkey://3des-cbc" => 8,
            _ => return Err(STATUS_INVALID_HANDLE),
        };
        if padding != 0 || flags & !BCRYPT_BLOCK_PADDING != 0 || (block == 1 && flags != 0) {
            return Err(STATUS_NOT_SUPPORTED);
        }
        let padded = flags & BCRYPT_BLOCK_PADDING != 0;
        if (decrypt || !padded) && !length.is_multiple_of(block) {
            return Err(STATUS_INVALID_PARAMETER);
        }
        if written == 0 || !emu.maps.validate_write_range(written, 4) {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let required = if padded && !decrypt {
            length
                .checked_add(block - length % block)
                .ok_or(STATUS_INVALID_PARAMETER)?
        } else {
            length
        };
        if required as u64 > emu.maps.max_alloc_size {
            return Err(STATUS_NOT_SUPPORTED);
        }
        if output == 0 {
            emu.maps.write_dword(written, required as u32);
            return Ok(());
        }
        if (!padded || !decrypt) && (capacity as u32 as usize) < required {
            return Err(STATUS_BUFFER_TOO_SMALL);
        }
        let mut data = cng_input(emu, input, length as u32)?;
        let mut chain = if iv != 0 {
            if iv_len as u32 as usize != block
                || block == 1
                || !emu.maps.validate_write_range(iv, block as u64)
            {
                return Err(STATUS_INVALID_PARAMETER);
            }
            cng_input(emu, iv, block as u32)?
        } else {
            if iv_len as u32 != 0 {
                return Err(STATUS_INVALID_PARAMETER);
            }
            vec![0; block]
        };
        if padded && !decrypt {
            data.resize(required, (required - length) as u8);
        }
        if block == 1 {
            if !crypto_state::encrypt_data(key, &mut data) {
                return Err(STATUS_INVALID_HANDLE);
            }
        } else {
            for chunk in data.chunks_exact_mut(block) {
                if decrypt {
                    let mut next = [0; 16];
                    next[..block].copy_from_slice(chunk);
                    if !crypto_state::decrypt_data(key, chunk) {
                        return Err(STATUS_INVALID_HANDLE);
                    }
                    for (byte, prev) in chunk.iter_mut().zip(&chain) {
                        *byte ^= prev;
                    }
                    chain.copy_from_slice(&next[..block]);
                } else {
                    for (byte, prev) in chunk.iter_mut().zip(&chain) {
                        *byte ^= prev;
                    }
                    if !crypto_state::encrypt_data(key, chunk) {
                        return Err(STATUS_INVALID_HANDLE);
                    }
                    chain.copy_from_slice(chunk);
                }
            }
            if padded && decrypt {
                let count = usize::from(*data.last().ok_or(STATUS_INVALID_PARAMETER)?);
                if count == 0
                    || count > block
                    || !data[data.len() - count..]
                        .iter()
                        .all(|&b| usize::from(b) == count)
                {
                    return Err(STATUS_INVALID_PARAMETER);
                }
                data.truncate(data.len() - count);
            }
        }
        if (capacity as u32 as usize) < data.len() {
            return Err(STATUS_BUFFER_TOO_SMALL);
        }
        if !data.is_empty() && !emu.maps.write_bytes(output, &data) {
            return Err(STATUS_INVALID_PARAMETER);
        }
        if iv != 0 {
            emu.maps.write_bytes(iv, &chain);
        }
        emu.maps.write_dword(written, data.len() as u32);
        Ok(())
    })();
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 10);
}
pub fn BCryptEncrypt(emu: &mut Emu) {
    cipher(emu, "BCryptEncrypt", false);
}
pub fn BCryptDecrypt(emu: &mut Emu) {
    cipher(emu, "BCryptDecrypt", true);
}

pub fn BCryptGetProperty(emu: &mut Emu) {
    const HASH_OBJECT_BYTES: u32 = 128;
    let call = WinApi(Stdcall);
    let [object, property, output, capacity, written, flags] = api_try!(emu, call.args(emu));
    let name = strings::read_text_checked(emu, property, true);
    log_red!(
        emu,
        "bcrypt!BCryptGetProperty object={object:#x} property={name:?} output={output:#x} capacity={capacity}"
    );
    let result = (|| {
        if flags as u32 != 0 || written == 0 || !emu.maps.validate_write_range(written, 4) {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let uri = helper::handler_get_uri(object);
        let alg = hash_algorithm(object)
            .ok()
            .map(|(n, _)| n)
            .or_else(|| {
                uri.strip_prefix("bcrypthash://")
                    .or_else(|| uri.strip_prefix("bcrypt-hmac://"))
                    .or_else(|| uri.strip_prefix("bcrypt://"))
            })
            .ok_or(STATUS_INVALID_HANDLE)?;
        let value = match (name.as_deref().ok_or(STATUS_INVALID_PARAMETER)?, alg) {
            ("HashDigestLength", "MD5") => 16,
            ("HashDigestLength", "SHA1") => 20,
            ("HashDigestLength", "SHA256") => 32,
            ("HashBlockLength", "MD5" | "SHA1" | "SHA256") => 64,
            ("ObjectLength", "MD5" | "SHA1" | "SHA256") => HASH_OBJECT_BYTES,
            ("BlockLength", "AES") => 16,
            ("BlockLength", "3DES") => 8,
            ("BlockLength", "RC4") => 1,
            _ => return Err(STATUS_NOT_SUPPORTED),
        };
        emu.maps.write_dword(written, 4);
        if output != 0 {
            if (capacity as u32) < 4 {
                return Err(STATUS_BUFFER_TOO_SMALL);
            }
            if !emu.maps.write_dword(output, value) {
                return Err(STATUS_INVALID_PARAMETER);
            }
        }
        Ok(())
    })();
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 6);
}
