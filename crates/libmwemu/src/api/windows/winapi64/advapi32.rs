use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64;
use crate::windows::constants;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "StartServiceCtrlDispatcherA" => StartServiceCtrlDispatcherA(emu),
        "StartServiceCtrlDispatcherW" => StartServiceCtrlDispatcherW(emu),
        "RegOpenKeyExA" => RegOpenKeyExA(emu),
        "RegQueryValueExA" => RegQueryValueExA(emu),
        "RegCloseKey" => RegCloseKey(emu),
        "GetUserNameA" => GetUserNameA(emu),
        "GetUserNameW" => GetUserNameW(emu),

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
