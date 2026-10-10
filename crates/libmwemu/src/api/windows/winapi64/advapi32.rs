use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64;
use crate::windows::constants;
use crate::windows::constants::*;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "StartServiceCtrlDispatcherA" => StartServiceCtrlDispatcherA(emu),
        "StartServiceCtrlDispatcherW" => StartServiceCtrlDispatcherW(emu),
        "RegOpenKeyExA" => RegOpenKeyExA(emu),
        "RegOpenKeyExW" => RegOpenKeyExW(emu),
        "RegQueryValueExA" => RegQueryValueExA(emu),
        "RegQueryValueExW" => RegQueryValueExW(emu),
        "RegCloseKey" => RegCloseKey(emu),
        "GetUserNameA" => GetUserNameA(emu),
        "GetUserNameW" => GetUserNameW(emu),
        "CryptAcquireContextA" => CryptAcquireContextA(emu),
        "CryptAcquireContextW" => CryptAcquireContextW(emu),
        "CryptReleaseContext" => CryptReleaseContext(emu),
        "CryptCreateHash" => CryptCreateHash(emu),
        "CryptHashData" => CryptHashData(emu),
        "CryptGenKey" => CryptGenKey(emu),
        "CryptEncrypt" => CryptEncrypt(emu),
        "CryptDecrypt" => CryptDecrypt(emu),
        "CryptGenRandom" => CryptGenRandom(emu),
        "AdjustTokenPrivileges" => AdjustTokenPrivileges(emu),

        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
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
    let service_table_entry_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("advapi32!StartServiceCtrlDispatcherA error reading service_table_entry pointer");

    let service_name = emu
        .maps
        .read_dword(service_table_entry_ptr as u64)
        .expect("advapi32!StartServiceCtrlDispatcherA error reading service_name");
    let service_name = emu
        .maps
        .read_dword((service_table_entry_ptr + 4) as u64)
        .expect("advapi32!StartServiceCtrlDispatcherA error reading service_name");

    emu.regs_mut().set_eax(1);
}

fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("advapi32!StartServiceCtrlDispatcherW error reading service_table_entry pointer");

    emu.regs_mut().set_eax(1);
}

fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let opts = emu.regs().r8;
    let result = emu.regs().r9;

    let subkey = emu.maps.read_string(subkey_ptr);

    log_red!(emu, "advapi32!RegOpenKeyExA {}", subkey);

    emu.maps
        .write_qword(result, helper::handler_create(&subkey));
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let opts = emu.regs().r8;
    let sam_desired = emu.regs().r9;
    let phk_result = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let subkey = emu.maps.read_wide_string(subkey_ptr);

    log_red!(emu, "advapi32!RegOpenKeyExW {}", subkey);

    if phk_result != 0 {
        let hndl = helper::handler_create(&format!("registry://{}", subkey));
        emu.maps.write_qword(phk_result, hndl);
    }
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

fn RegCloseKey(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;

    log_red!(emu, "advapi32!RegCloseKey");

    helper::handler_close(hkey);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

fn RegQueryValueExA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let typ_out = emu.regs().r9;
    let data_out = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("error reading api aparam");
    let datasz_out = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("error reading api param");

    let mut value = String::new();
    if value_ptr > 0 {
        value = emu.maps.read_string(value_ptr);
    }

    log_red!(emu, "advapi32!RegQueryValueExA {}", value);

    if data_out > 0 {
        emu.maps.write_string(data_out, "some_random_reg_contents");
    }
    if datasz_out > 0 {
        emu.maps.write_qword(datasz_out, 24);
    }
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let typ_out = emu.regs().r9;
    let data_out = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let datasz_out = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let mut value = String::new();
    if value_ptr > 0 {
        value = emu.maps.read_wide_string(value_ptr);
    }

    log_red!(emu, "advapi32!RegQueryValueExW {}", value);

    if typ_out != 0 {
        emu.maps.write_dword(typ_out, 1); // REG_SZ
    }
    if data_out > 0 && datasz_out > 0 {
        emu.maps.write_wide_string(data_out, "");
        let written: u64 = 2; // empty wide string = null terminator (2 bytes)
        emu.maps.write_dword(datasz_out, written as u32);
    }
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

fn GetUserNameA(emu: &mut emu::Emu) {
    let out_username = emu.regs().rcx; // LPSTR lpBuffer
    let in_out_sz = emu.regs().rdx; // LP64WORD pcbBuffer (your 64-bit variant)

    log_red!(
        emu,
        "advapi32!GetUserNameA lpBuffer: 0x{:x} pcbBuffer: 0x{:x}",
        out_username,
        in_out_sz
    );

    // Check if size pointer is valid
    if in_out_sz == 0 || !emu.maps.is_mapped(in_out_sz) {
        log_red!(emu, "GetUserNameA: Invalid pcbBuffer pointer");
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read current buffer size (in bytes)
    let buffer_size = emu
        .maps
        .read_qword(in_out_sz)
        .expect("Cannot read buffer size") as usize;

    // Calculate required size in bytes (including null terminator)
    let user_name = emu.cfg.user_name.clone();
    let required_size = user_name.len() + 1; // +1 for null terminator

    // Always update the size to show required bytes
    emu.maps.write_qword(in_out_sz, required_size as u64);

    // Check if output buffer is valid
    if out_username == 0 || !emu.maps.is_mapped(out_username) {
        log_red!(emu, "GetUserNameA: Invalid lpBuffer pointer");
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Check if buffer is large enough
    if buffer_size < required_size {
        log_red!(
            emu,
            "GetUserNameA: Buffer too small. Required: {}, Provided: {}",
            required_size,
            buffer_size
        );
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Buffer is large enough, write the username
    emu.maps.write_string(out_username, &user_name);

    log_red!(
        emu,
        "GetUserNameA returning: '{}' (size: {})",
        user_name,
        required_size
    );

    emu.regs_mut().rax = constants::TRUE;
}

fn GetUserNameW(emu: &mut emu::Emu) {
    let out_username = emu.regs().rcx; // LPWSTR lpBuffer
    let in_out_sz = emu.regs().rdx; // LPDWORD pcbBuffer

    log_red!(
        emu,
        "advapi32!GetUserNameW lpBuffer: 0x{:x} pcbBuffer: 0x{:x}",
        out_username,
        in_out_sz
    );

    // Check if size pointer is valid
    if in_out_sz == 0 || !emu.maps.is_mapped(in_out_sz) {
        log_red!(emu, "GetUserNameW: Invalid pcbBuffer pointer");
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read current buffer size (in characters)
    let buffer_size = emu
        .maps
        .read_dword(in_out_sz)
        .expect("Cannot read buffer size") as usize;

    // Calculate required size in characters (including null terminator)
    let user_name = emu.cfg.user_name.clone();
    let username_chars = user_name.chars().count();
    let required_size = username_chars + 1; // +1 for null terminator

    // Always update the size to show required characters
    emu.maps.write_dword(in_out_sz, required_size as u32);

    // Check if output buffer is valid
    if out_username == 0 || !emu.maps.is_mapped(out_username) {
        log_red!(emu, "GetUserNameW: Invalid lpBuffer pointer");
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Check if buffer is large enough
    if buffer_size < required_size {
        log_red!(
            emu,
            "GetUserNameW: Buffer too small. Required: {}, Provided: {}",
            required_size,
            buffer_size
        );
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Buffer is large enough, write the username
    emu.maps.write_wide_string(out_username, &user_name);

    log_red!(
        emu,
        "GetUserNameW returning: '{}' (size: {})",
        user_name,
        required_size
    );

    emu.regs_mut().rax = constants::TRUE;
}

///// CRYPTO API /////

pub fn CryptAcquireContextA(emu: &mut emu::Emu) {
    let out_handle = emu.regs().rcx;
    let container_ptr = emu.regs().rdx;
    let provider_ptr = emu.regs().r8;
    let prov_type = emu.regs().r9 as u32;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as u32;

    let uri = "cryptctx://".to_string();
    let hndl = helper::handler_create(&uri);
    if out_handle != 0 {
        emu.maps.write_qword(out_handle, hndl);
    }

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
        "advapi32!CryptAcquireContextA =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    emu.regs_mut().rax = 1;
}

pub fn CryptAcquireContextW(emu: &mut emu::Emu) {
    let out_handle = emu.regs().rcx;
    let container_ptr = emu.regs().rdx;
    let provider_ptr = emu.regs().r8;
    let prov_type = emu.regs().r9 as u32;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as u32;

    let uri = "cryptctx://".to_string();
    let hndl = helper::handler_create(&uri);
    if out_handle != 0 {
        emu.maps.write_qword(out_handle, hndl);
    }

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
        "advapi32!CryptAcquireContextW =0x{:x} type: {} flags: `{}`",
        hndl,
        prov_type,
        &sflags
    );

    emu.regs_mut().rax = 1;
}

pub fn CryptReleaseContext(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;

    log_red!(emu, "advapi32!CryptReleaseContext hProv=0x{:x}", hprov);

    helper::handler_close(hprov);

    emu.regs_mut().rax = 1;
}

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let alg_id = emu.regs().rdx as u32;
    let hkey = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let ph_hash = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let alg_name = constants::get_cryptoalgorithm_name(alg_id);
    let uri = format!("crypthash://{}", alg_name);
    let hndl = helper::handler_create(&uri);

    if ph_hash != 0 {
        emu.maps.write_qword(ph_hash, hndl);
    }

    log_red!(
        emu,
        "advapi32!CryptCreateHash alg={} =0x{:x}",
        alg_name,
        hndl
    );

    emu.regs_mut().rax = 1;
}

pub fn CryptHashData(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    let data_ptr = emu.regs().rdx;
    let data_len = emu.regs().r8;
    let flags = emu.regs().r9 as u32;

    log_red!(
        emu,
        "advapi32!CryptHashData hHash=0x{:x} data=0x{:x} len={}",
        hhash,
        data_ptr,
        data_len
    );

    emu.regs_mut().rax = 1;
}

pub fn CryptGenKey(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let alg_id = emu.regs().rdx as u32;
    let flags = emu.regs().r8;
    let ph_key = emu.regs().r9;

    let alg_name = constants::get_cryptoalgorithm_name(alg_id);
    let uri = format!("cryptkey://{}", alg_name);
    let hndl = helper::handler_create(&uri);

    if ph_key != 0 {
        emu.maps.write_qword(ph_key, hndl);
    }

    log_red!(emu, "advapi32!CryptGenKey alg={} =0x{:x}", alg_name, hndl);

    emu.regs_mut().rax = 1;
}

pub fn CryptEncrypt(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let hhash = emu.regs().rdx;
    let bfinal = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let data_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let buff_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(emu, "advapi32!CryptEncrypt hKey=0x{:x}", hkey);

    emu.regs_mut().rax = 1;
}

pub fn CryptDecrypt(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let hhash = emu.regs().rdx;
    let bfinal = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let data_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(emu, "advapi32!CryptDecrypt hKey=0x{:x}", hkey);

    emu.regs_mut().rax = 1;
}

pub fn CryptGenRandom(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let dw_len = emu.regs().rdx;
    let pb_buffer = emu.regs().r8;

    log_red!(
        emu,
        "advapi32!CryptGenRandom len={} buf=0x{:x}",
        dw_len,
        pb_buffer
    );

    if pb_buffer != 0 && dw_len > 0 {
        let tick = emu.tick as u8;
        for i in 0..dw_len {
            let byte = tick.wrapping_add(i as u8);
            emu.maps.write_byte(pb_buffer + i, byte);
        }
    }

    emu.regs_mut().rax = 1;
}

pub fn AdjustTokenPrivileges(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rcx;
    let disable_all = emu.regs().rdx;
    let new_state = emu.regs().r8;
    let buffer_length = emu.regs().r9 as u32;
    let previous_state = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let return_length = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(
        emu,
        "advapi32!AdjustTokenPrivileges handle=0x{:x} disableAll={}",
        token_handle,
        disable_all
    );

    emu.regs_mut().rax = 1;
}
