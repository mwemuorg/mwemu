use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants::*;
use md5;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "StartServiceCtrlDispatcherA" => StartServiceCtrlDispatcherA(emu),
        "StartServiceCtrlDispatcherW" => StartServiceCtrlDispatcherW(emu),
        "LookupPrivilegeValueW" => LookupPrivilegeValueW(emu),
        "CryptAcquireContextA" => CryptAcquireContextA(emu),
        "CryptAcquireContextW" => CryptAcquireContextW(emu),
        "CryptEncrypt" => CryptEncrypt(emu),
        "CryptDecrypt" => CryptDecrypt(emu),
        "CryptCreateHash" => CryptCreateHash(emu),
        "CryptGenKey" => CryptGenKey(emu),
        "CryptGetHashParam" => CryptGetHashParam(emu),
        "CryptGetKeyParam" => CryptGetKeyParam(emu),
        "CryptImportKey" => CryptImportKey(emu),
        "CryptSignHashA" => CryptSignHashA(emu),
        "CryptSignHashW" => CryptSignHashW(emu),
        "CryptReleaseContext" => CryptReleaseContext(emu),
        "CryptHashData" => CryptHashData(emu),
        "CryptDeriveKey" => CryptDeriveKey(emu),

        "RegOpenKeyExA" => RegOpenKeyExA(emu),
        "RegQueryValueExA" => RegQueryValueExA(emu),
        "RegQueryValueExW" => RegQueryValueExW(emu),
        "GetUserNameA" => GetUserNameA(emu),
        "GetUserNameW" => GetUserNameW(emu),
        "CryptGenRandom" => CryptGenRandom(emu),
        "AdjustTokenPrivileges" => AdjustTokenPrivileges(emu),
        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

fn StartServiceCtrlDispatcherA(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    /*
    let service_name = emu.maps.read_dword(service_table_entry_ptr as u64)
        .unwrap_or(0);
    let service_name = emu.maps.read_dword((service_table_entry_ptr+4) as u64)
        .unwrap_or(0);*/

    log_red!(emu, "advapi321!StartServiceCtrlDispatcherA");

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}

fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "advapi321!StartServiceCtrlDispatcherW");

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}

///// CRYPTO API /////

fn CryptAcquireContextA(emu: &mut emu::Emu) {
    let out_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let container = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let provider = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let uri = "cryptctx://".to_string();
    let hndl = helper::handler_create(&uri) as u32;
    emu.maps.write_dword(out_handle, hndl);

    let mut sflags = String::new();
    if flags & CRYPT_VERIFYCONTEXT == CRYPT_VERIFYCONTEXT {
        sflags.push_str("CRYPT_VERIFYCONTEXT ");
    }
    if flags & CRYPT_NEWKEYSET == CRYPT_NEWKEYSET {
        sflags.push_str("CRYPT_NEWKEYSET ");
    }
    if flags & CRYPT_DELETEKEYSET == CRYPT_DELETEKEYSET {
        sflags.push_str("CRYPT_DELETEKEYSET ");
    }
    if flags & CRYPT_MACHINE_KEYSET == CRYPT_MACHINE_KEYSET {
        sflags.push_str("CRYPT_MACHINE_KEYSET ");
    }
    if flags & CRYPT_SILENT == CRYPT_SILENT {
        sflags.push_str("CRYPT_SILENT ");
    }
    if flags & CRYPT_DEFAULT_CONTAINER_OPTIONAL == CRYPT_DEFAULT_CONTAINER_OPTIONAL {
        sflags.push_str("CRYPT_DEFAULT_CONTAINER_OPTIONAL ");
    }

    log_red!(
        emu,
        "advapi321!CryptAcquireContextA =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptAcquireContextW(emu: &mut emu::Emu) {
    let out_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let container = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let provider = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let uri = "cryptctx://".to_string();
    let hndl = helper::handler_create(&uri) as u32;
    emu.maps.write_dword(out_handle, hndl);

    let mut sflags = String::new();
    if flags & CRYPT_VERIFYCONTEXT == CRYPT_VERIFYCONTEXT {
        sflags.push_str("CRYPT_VERIFYCONTEXT ");
    }
    if flags & CRYPT_NEWKEYSET == CRYPT_NEWKEYSET {
        sflags.push_str("CRYPT_NEWKEYSET ");
    }
    if flags & CRYPT_DELETEKEYSET == CRYPT_DELETEKEYSET {
        sflags.push_str("CRYPT_DELETEKEYSET ");
    }
    if flags & CRYPT_MACHINE_KEYSET == CRYPT_MACHINE_KEYSET {
        sflags.push_str("CRYPT_MACHINE_KEYSET ");
    }
    if flags & CRYPT_SILENT == CRYPT_SILENT {
        sflags.push_str("CRYPT_SILENT ");
    }
    if flags & CRYPT_DEFAULT_CONTAINER_OPTIONAL == CRYPT_DEFAULT_CONTAINER_OPTIONAL {
        sflags.push_str("CRYPT_DEFAULT_CONTAINER_OPTIONAL ");
    }

    log_red!(
        emu,
        "advapi321!CryptAcquireContextW =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn LookupPrivilegeValueW(emu: &mut emu::Emu) {
    let ptr_sysname = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let ptr_name = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let ptr_uid = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    let sysname = emu.maps.read_wide_string(ptr_sysname);
    let name = emu.maps.read_wide_string(ptr_name);
    emu.maps.write_dword(ptr_uid, 123);

    log_red!(
        emu,
        "advapi321!LookupPrivilegeValueW `{}` `{}`",
        sysname,
        name
    );

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptEncrypt(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let hhash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let bfinal = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let buff_len = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptEncrypt");

    for _ in 0..7 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}

fn CryptDecrypt(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let hhash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let bfinal = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let data_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let buff_len = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptDecrypt");

    for _ in 0..7 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}

fn CryptReleaseContext(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptReleaseContext");

    helper::handler_close(hndl);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn CryptCreateHash(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let hkey = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let hash_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    log_red!(
        emu,
        "advapi32!CryptCreateHash {}",
        get_cryptoalgorithm_name(algid)
    );

    let hndl = helper::handler_create(get_cryptoalgorithm_name(algid));
    assert!(hndl < 0x00000001_00000000);
    emu.maps.write_dword(hash_ptr, hndl as u32);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptGenKey(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let hkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(
        emu,
        "advapi32!CryptGenKey {}",
        get_cryptoalgorithm_name(algid)
    );

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptGetHashParam(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptGetHashParam");

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptGetKeyParam(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let param = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptGetKeyParam");

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptImportKey(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let hpubkey = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let hkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptImportKey");

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptSignHashA(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let sig_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptSignHashA");

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptSignHashW(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let desc_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let sig_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let sig_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    log_red!(emu, "advapi32!CryptSignHashW");

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptHashData(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let data_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let data_len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let data = emu.maps.read_bytes(data_ptr, data_len);

    let mut hex_hash = "".to_string();
    let algo = helper::handler_get_uri(hhash);
    if algo == "CALG_MD5" {
        let digest: md5::Digest = md5::compute(data);
        let hash_bytes = digest.0;
        hex_hash = format!("{:x}", digest);
        helper::handler_put_bytes(hhash, &hash_bytes);
    } else {
        helper::handler_put_bytes(hhash, b"deadcafebabe");
    }

    log_red!(emu, "advapi32!CryptHashData {}", hex_hash);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

fn CryptDeriveKey(emu: &mut emu::Emu) {
    let hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let data = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as usize;
    let hkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

    let alg = get_cryptoalgorithm_name(algid);
    let mut alg_len = get_crypto_key_len(algid);

    let handle = helper::handler_create(alg);
    if alg_len == 0 {
        alg_len = (flags >> 16) & 0xffff;
    }
    helper::handler_put_bytes(handle, &vec![0x41u8; alg_len]);

    log_red!(emu, "advapi32!CryptDeriveKey {}", alg);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let subkey = emu.maps.read_string(arg(emu, 1) as u64);
    let result_ptr = arg(emu, 4) as u64;
    log_red!(emu, "advapi32!RegOpenKeyExA {}", subkey);
    if result_ptr != 0 {
        let hndl = helper::handler_create(&format!("registry://{}", subkey));
        emu.maps.write_dword(result_ptr, hndl as u32);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 5);
}
pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    let value_ptr = arg(emu, 1) as u64;
    let type_ptr = arg(emu, 3) as u64;
    let data_ptr = arg(emu, 4) as u64;
    let data_size_ptr = arg(emu, 5) as u64;
    let value = if value_ptr != 0 {
        emu.maps.read_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!RegQueryValueExA `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    // The registry is not modelled: hand back a well-formed empty value.
    if data_ptr != 0 {
        emu.maps.write_string(data_ptr, "");
    }
    if data_size_ptr != 0 {
        emu.maps.write_dword(data_size_ptr, 1);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 6);
}
fn RegQueryValueExW(emu: &mut emu::Emu) {
    let value_ptr = arg(emu, 1) as u64;
    let type_ptr = arg(emu, 3) as u64;
    let data_ptr = arg(emu, 4) as u64;
    let data_size_ptr = arg(emu, 5) as u64;
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!RegQueryValueExW `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    // The registry is not modelled: hand back a well-formed empty value.
    if data_ptr != 0 {
        emu.maps.write_wide_string(data_ptr, "");
    }
    if data_size_ptr != 0 {
        emu.maps.write_dword(data_size_ptr, 2);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 6);
}
fn GetUserNameA(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let size_ptr = arg(emu, 1) as u64;
    let user = emu.cfg.user_name.clone();
    let required = user.chars().count() as u32 + 1;
    let provided = emu.maps.read_dword(size_ptr).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!GetUserNameA '{}' buf size: {}",
        user,
        provided
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, required);
    }
    if buf == 0 || size_ptr == 0 || provided < required {
        emu.regs_mut().rax = 0;
    } else {
        emu.maps.write_string(buf, &user);
        emu.regs_mut().rax = 1;
    }
    stdcall_ret(emu, 2);
}
fn GetUserNameW(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let size_ptr = arg(emu, 1) as u64;
    let user = emu.cfg.user_name.clone();
    let required = user.chars().count() as u32 + 1;
    let provided = emu.maps.read_dword(size_ptr).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!GetUserNameW '{}' buf size: {}",
        user,
        provided
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, required);
    }
    if buf == 0 || size_ptr == 0 || provided < required {
        emu.regs_mut().rax = 0;
    } else {
        emu.maps.write_wide_string(buf, &user);
        emu.regs_mut().rax = 1;
    }
    stdcall_ret(emu, 2);
}
fn CryptGenRandom(emu: &mut emu::Emu) {
    let len = arg(emu, 1) as u64;
    let buf = arg(emu, 2) as u64;
    log_red!(emu, "advapi32!CryptGenRandom len={} buf=0x{:x}", len, buf);
    let tick = emu.tick as u8;
    for i in 0..len {
        emu.maps.write_byte(buf + i, tick.wrapping_add(i as u8));
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let token = arg(emu, 0);
    let disable_all = arg(emu, 1);
    log_red!(
        emu,
        "advapi32!AdjustTokenPrivileges token=0x{:x} disableAll={}",
        token,
        disable_all
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
