use super::provider_crypto::{BASE_PROVIDER_NAME, PROVIDER_URI, is_csp_hash, is_csp_key};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory;
use crate::winapi::common::trace;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::crypto::*;
use crate::windows::constants::errors::*;

fn return_bool(emu: &mut Emu, result: Result<(), u64>, words: usize) {
    let call = WinApi(Stdcall);
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, words);
}

fn destroy(emu: &mut Emu, name: &str, is_hash: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let ok = if is_hash {
        is_csp_hash(emu, handle) && emu.crypto.destroy_hash(handle)
    } else {
        is_csp_key(emu, handle) && emu.crypto.destroy_key(handle)
    };
    if ok {
        emu.crypto.close(handle);
    }
    log_red!(emu, "advapi32!{name} handle={handle:#x}");
    return_bool(
        emu,
        if ok {
            Ok(())
        } else {
            Err(ERROR_INVALID_HANDLE)
        },
        1,
    );
}
pub fn CryptDestroyKey(emu: &mut Emu) {
    destroy(emu, "CryptDestroyKey", false);
}
pub fn CryptDestroyHash(emu: &mut Emu) {
    destroy(emu, "CryptDestroyHash", true);
}

fn parameter(emu: &mut Emu, name: &str, is_hash: bool) {
    let call = WinApi(Stdcall);
    let [handle, parameter, output, size, flags] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "advapi32!{name} handle={handle:#x} parameter={parameter} output={output:#x} size={size:#x} flags={flags:#x}"
    );
    let result = (|| {
        if flags as u32 != 0 || size == 0 || !emu.maps.validate_write_range(size, 4) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if (is_hash && !is_csp_hash(emu, handle)) || (!is_hash && !is_csp_key(emu, handle)) {
            return Err(ERROR_INVALID_HANDLE);
        }
        let capacity = if output == 0 {
            None
        } else {
            Some(emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)? as usize)
        };
        if is_hash && parameter as u32 == HP_HASHVAL {
            let bytes = emu
                .crypto
                .get_hash_param(handle, HP_HASHSIZE)
                .ok_or(ERROR_INVALID_HANDLE)?;
            let length = u32::from_le_bytes(bytes.try_into().map_err(|_| ERROR_INVALID_PARAMETER)?);
            emu.maps.write_dword(size, length);
            if output == 0 {
                return Ok(());
            }
            if capacity.is_some_and(|available| available < length as usize) {
                return Err(ERROR_MORE_DATA);
            }
            if !emu.maps.validate_write_range(output, u64::from(length)) {
                return Err(ERROR_NOACCESS);
            }
        }
        let bytes = if is_hash {
            emu.crypto.get_hash_param(handle, parameter as u32)
        } else {
            emu.crypto.get_key_param(handle, parameter as u32)
        }
        .ok_or(ERROR_INVALID_PARAMETER)?;
        emu.maps.write_dword(size, bytes.len() as u32);
        if let Some(capacity) = capacity {
            if capacity < bytes.len() {
                return Err(ERROR_MORE_DATA);
            }
            if !emu.maps.write_bytes(output, &bytes) {
                return Err(ERROR_NOACCESS);
            }
        }
        Ok(())
    })();
    return_bool(emu, result, 5);
}
pub fn CryptGetHashParam(emu: &mut Emu) {
    parameter(emu, "CryptGetHashParam", true);
}
pub fn CryptGetKeyParam(emu: &mut Emu) {
    parameter(emu, "CryptGetKeyParam", false);
}
pub fn CryptGenKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, alg_id, flags, output] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "advapi32!CryptGenKey provider={provider:#x} algorithm={alg_id:#x} flags={flags:#x} output={output:#x}"
    );
    let bits = flags as u32 >> 16;
    if alg_id as u32 != CALG_RC4
        || (flags as u32 & 0xffff & !CRYPT_EXPORTABLE) != 0
        || (bits != 0 && (!bits.is_multiple_of(8) || !(40..=2048).contains(&bits)))
    {
        policy::unsupported(emu, "advapi32", "CryptGenKey", emu.pc());
        return;
    }
    let result = if emu.crypto.name(provider) != PROVIDER_URI {
        Err(ERROR_INVALID_HANDLE)
    } else if !memory::writable_pointer(emu, output) {
        Err(ERROR_NOACCESS)
    } else {
        let handle = emu.crypto.open("cryptkey:rc4");
        let length = if bits == 0 { 16 } else { bits / 8 } as usize;
        let bytes: Vec<u8> = (0..length.div_ceil(8))
            .flat_map(|_| emu.random_u64().to_le_bytes())
            .take(length)
            .collect();
        emu.crypto.create_key(handle, CALG_RC4, (length * 8) as u32);
        emu.crypto.set_key_data(handle, &bytes);
        emu.crypto
            .set_exportable(handle, flags as u32 & CRYPT_EXPORTABLE != 0);
        memory::write_pointer(emu, output, handle);
        Ok(())
    };
    return_bool(emu, result, 4);
}
pub fn CryptDeriveKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, alg_id, base_hash, flags, output] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "advapi32!CryptDeriveKey provider={provider:#x} algorithm={alg_id:#x} hash={base_hash:#x} flags={flags:#x} output={output:#x}"
    );
    if alg_id as u32 != CALG_RC4 || flags as u32 & !CRYPT_EXPORTABLE != 0 {
        policy::unsupported(emu, "advapi32", "CryptDeriveKey", emu.pc());
        return;
    }
    let result = if emu.crypto.name(provider) != PROVIDER_URI || !is_csp_hash(emu, base_hash) {
        Err(ERROR_INVALID_HANDLE)
    } else if !memory::writable_pointer(emu, output) {
        Err(ERROR_NOACCESS)
    } else {
        let handle = emu.crypto.open("derivedkey:rc4");
        if emu.crypto.derive_key_from_hash(handle, base_hash, CALG_RC4) {
            emu.crypto
                .set_exportable(handle, flags as u32 & CRYPT_EXPORTABLE != 0);
            memory::write_pointer(emu, output, handle);
            Ok(())
        } else {
            emu.crypto.close(handle);
            Err(NTE_BAD_HASH_STATE)
        }
    };
    return_bool(emu, result, 5);
}

pub fn CryptEnumProviderTypesA(emu: &mut Emu) {
    provider_name(emu, "CryptEnumProviderTypesA", Encoding::Ansi, true, true);
}

pub fn CryptEnumProviderTypesW(emu: &mut Emu) {
    provider_name(emu, "CryptEnumProviderTypesW", Encoding::Wide, true, true);
}

pub fn CryptEnumProvidersA(emu: &mut Emu) {
    provider_name(emu, "CryptEnumProvidersA", Encoding::Ansi, true, false);
}

pub fn CryptEnumProvidersW(emu: &mut Emu) {
    provider_name(emu, "CryptEnumProvidersW", Encoding::Wide, true, false);
}

pub fn CryptGetDefaultProviderA(emu: &mut Emu) {
    provider_name(
        emu,
        "CryptGetDefaultProviderA",
        Encoding::Ansi,
        false,
        false,
    );
}

pub fn CryptGetDefaultProviderW(emu: &mut Emu) {
    provider_name(
        emu,
        "CryptGetDefaultProviderW",
        Encoding::Wide,
        false,
        false,
    );
}

const BASE_PROVIDER_TYPE_NAME: &str = "RSA Full (Signature and Key Exchange)";

fn put_bytes(emu: &mut Emu, output: u64, size: u64, data: &[u8]) -> Result<(), u64> {
    if size == 0 || !emu.maps.validate_write_range(size, 4) {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let capacity = emu.maps.read_dword(size).ok_or(ERROR_NOACCESS)? as usize;
    emu.maps.write_dword(size, data.len() as u32);
    if output != 0 {
        if capacity < data.len() {
            return Err(ERROR_MORE_DATA);
        }
        if !emu.maps.validate_write_range(output, data.len() as u64)
            || !emu.maps.write_bytes(output, data)
        {
            return Err(ERROR_NOACCESS);
        }
    }
    Ok(())
}

fn provider_name(emu: &mut Emu, api: &str, encoding: Encoding, enumerate: bool, type_name: bool) {
    let call = WinApi(Stdcall);
    let [selector, reserved, flags] = api_try!(emu, call.args(emu));
    let (kind_out, output, size) = if enumerate {
        (
            api_try!(emu, call.arg(emu, 3)),
            api_try!(emu, call.arg(emu, 4)),
            api_try!(emu, call.arg(emu, 5)),
        )
    } else {
        (
            0,
            api_try!(emu, call.arg(emu, 3)),
            api_try!(emu, call.arg(emu, 4)),
        )
    };
    trace::export(emu, call, "advapi32", api);
    let result = (|| {
        if reserved != 0 || (enumerate && flags as u32 != 0) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if enumerate && selector as u32 != 0 {
            return Err(ERROR_NO_MORE_ITEMS);
        }
        if !enumerate && selector as u32 != PROV_RSA_FULL {
            return Err(NTE_PROV_TYPE_NOT_DEF);
        }
        if !enumerate && !matches!(flags as u32, 0 | CRYPT_MACHINE_DEFAULT | CRYPT_USER_DEFAULT) {
            return Err(NTE_BAD_FLAGS);
        }
        if enumerate && (kind_out == 0 || !emu.maps.validate_write_range(kind_out, 4)) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let name = if type_name {
            BASE_PROVIDER_TYPE_NAME
        } else {
            BASE_PROVIDER_NAME
        };
        let bytes = encoding.encode(&name.encode_utf16().chain([0]).collect::<Vec<_>>());
        put_bytes(emu, output, size, &bytes)?;
        if enumerate {
            emu.maps.write_dword(kind_out, PROV_RSA_FULL);
        }
        Ok(())
    })();
    return_bool(emu, result, if enumerate { 6 } else { 5 });
}

pub fn CryptGetProvParam(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, parameter, output, size, flags] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", "CryptGetProvParam");
    if !matches!(parameter as u32, PP_NAME | PP_PROVTYPE) || flags as u32 != 0 {
        policy::unsupported(emu, "advapi32", "CryptGetProvParam", emu.pc());
        return;
    }
    let result = if emu.crypto.name(provider) != PROVIDER_URI {
        Err(ERROR_INVALID_HANDLE)
    } else if parameter as u32 == PP_PROVTYPE {
        put_bytes(emu, output, size, &PROV_RSA_FULL.to_le_bytes())
    } else {
        let mut bytes = BASE_PROVIDER_NAME.as_bytes().to_vec();
        bytes.push(0);
        put_bytes(emu, output, size, &bytes)
    };
    return_bool(emu, result, 5);
}

fn copy_object(emu: &mut Emu, api: &str, is_hash: bool) {
    let call = WinApi(Stdcall);
    let [source, reserved, flags, output] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", api);
    let result = (|| {
        if reserved != 0 || flags as u32 != 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if !(if is_hash {
            is_csp_hash(emu, source)
        } else {
            is_csp_key(emu, source)
        }) {
            return Err(ERROR_INVALID_HANDLE);
        }
        if output == 0 || !memory::writable_pointer(emu, output) {
            return Err(ERROR_NOACCESS);
        }
        let copy = emu.crypto.open(emu.crypto.name(source).to_owned());
        if !emu.crypto.duplicate_object(source, copy, is_hash) {
            emu.crypto.close(copy);
            return Err(ERROR_INVALID_HANDLE);
        }
        memory::write_pointer(emu, output, copy);
        Ok(())
    })();
    return_bool(emu, result, 4);
}
pub fn CryptDuplicateHash(emu: &mut Emu) {
    copy_object(emu, "CryptDuplicateHash", true);
}
pub fn CryptDuplicateKey(emu: &mut Emu) {
    copy_object(emu, "CryptDuplicateKey", false);
}

const KEY_BLOB_VERSION: u8 = 2;
const PLAINTEXT_KEY_HEADER: usize = 12;
const MAX_RC4_KEY_BYTES: usize = 256;

pub fn CryptImportKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [provider, blob, length, wrapping_key, flags, output] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", "CryptImportKey");
    let size = length as u32 as usize;
    let result = (|| {
        if emu.crypto.name(provider) != PROVIDER_URI {
            return Err(ERROR_INVALID_HANDLE);
        }
        if output == 0 || !memory::writable_pointer(emu, output) {
            return Err(ERROR_NOACCESS);
        }
        if size < PLAINTEXT_KEY_HEADER {
            return Err(NTE_BAD_DATA);
        }
        let hdr = memory::read_array::<PLAINTEXT_KEY_HEADER>(emu, blob).ok_or(ERROR_NOACCESS)?;
        let algorithm = u32::from_le_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]);
        if hdr[0] != PLAINTEXTKEYBLOB
            || algorithm != CALG_RC4
            || wrapping_key != 0
            || flags as u32 & !CRYPT_EXPORTABLE != 0
        {
            return Ok(None);
        }
        let key_size = u32::from_le_bytes([hdr[8], hdr[9], hdr[10], hdr[11]]) as usize;
        if hdr[1] != KEY_BLOB_VERSION
            || hdr[2..4] != [0, 0]
            || !(5..=MAX_RC4_KEY_BYTES).contains(&key_size)
            || size - PLAINTEXT_KEY_HEADER != key_size
        {
            return Err(NTE_BAD_DATA);
        }
        let start = blob
            .checked_add(PLAINTEXT_KEY_HEADER as u64)
            .ok_or(ERROR_NOACCESS)?;
        let bytes = emu
            .maps
            .try_read_bytes(start, key_size)
            .ok_or(ERROR_NOACCESS)?
            .into_owned();
        let handle = emu.crypto.open("cryptkey:rc4");
        emu.crypto
            .create_key(handle, CALG_RC4, (key_size * 8) as u32);
        emu.crypto.set_key_data(handle, &bytes);
        emu.crypto
            .set_exportable(handle, flags as u32 & CRYPT_EXPORTABLE != 0);
        memory::write_pointer(emu, output, handle);
        Ok(Some(()))
    })();
    match result {
        Ok(None) => {
            policy::unsupported(emu, "advapi32", "CryptImportKey", emu.pc());
        }
        other => return_bool(emu, other.map(|_| ()), 6),
    }
}
pub fn CryptExportKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [source, wrapping_key, blob_type, flags, output, size] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", "CryptExportKey");
    if blob_type as u32 != u32::from(PLAINTEXTKEYBLOB) || wrapping_key != 0 || flags as u32 != 0 {
        policy::unsupported(emu, "advapi32", "CryptExportKey", emu.pc());
        return;
    }
    let result = (|| {
        if !is_csp_key(emu, source) {
            return Err(ERROR_INVALID_HANDLE);
        }
        if !emu.crypto.is_exportable(source) {
            return Err(NTE_BAD_KEY_STATE);
        }
        let material = emu.crypto.export_key(source).ok_or(NTE_BAD_KEY)?;
        let mut blob = Vec::with_capacity(PLAINTEXT_KEY_HEADER + material.len());
        blob.extend([PLAINTEXTKEYBLOB, KEY_BLOB_VERSION, 0, 0]);
        blob.extend(CALG_RC4.to_le_bytes());
        blob.extend((material.len() as u32).to_le_bytes());
        blob.extend(material);
        put_bytes(emu, output, size, &blob)
    })();
    return_bool(emu, result, 6);
}
