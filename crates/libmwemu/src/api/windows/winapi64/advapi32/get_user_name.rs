use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::system::*;

pub(super) fn GetUserNameA(emu: &mut emu::Emu) {
    get_user_name(emu, "GetUserNameA", false);
}

pub(super) fn GetUserNameW(emu: &mut emu::Emu) {
    get_user_name(emu, "GetUserNameW", true);
}

fn get_user_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let out_username = emu.regs().rcx;
    let in_out_sz = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!{api} lpBuffer: 0x{:x} pcbBuffer: 0x{:x}",
        out_username,
        in_out_sz
    );
    if in_out_sz == 0 || !emu.maps.is_mapped(in_out_sz) {
        log_red!(emu, "{api}: Invalid pcbBuffer pointer");
        emu.regs_mut().rax = FALSE;
        return;
    }
    if wide {
        let buffer_size = emu.maps.read_dword(in_out_sz).unwrap_or(0) as usize;
        let user_name = emu.cfg.user_name.clone();
        let username_chars = user_name.chars().count();
        let required_size = username_chars + 1;
        emu.maps.write_dword(in_out_sz, required_size as u32);
        if out_username == 0 || !emu.maps.is_mapped(out_username) {
            log_red!(emu, "{api}: Invalid lpBuffer pointer");
            emu.regs_mut().rax = FALSE;
            return;
        }
        if buffer_size < required_size {
            log_red!(
                emu,
                "{api}: Buffer too small. Required: {}, Provided: {}",
                required_size,
                buffer_size
            );
            emu.regs_mut().rax = FALSE;
            return;
        }
        strings::write_string(emu, out_username, &user_name, wide);
        log_red!(
            emu,
            "{api} returning: '{}' (size: {})",
            user_name,
            required_size
        );
        emu.regs_mut().rax = TRUE;
    } else {
        let buffer_size = emu.maps.read_qword(in_out_sz).unwrap_or(0) as usize;
        let user_name = emu.cfg.user_name.clone();
        let required_size = user_name.len() + 1;
        emu.maps.write_qword(in_out_sz, required_size as u64);
        if out_username == 0 || !emu.maps.is_mapped(out_username) {
            log_red!(emu, "{api}: Invalid lpBuffer pointer");
            emu.regs_mut().rax = FALSE;
            return;
        }
        if buffer_size < required_size {
            log_red!(
                emu,
                "{api}: Buffer too small. Required: {}, Provided: {}",
                required_size,
                buffer_size
            );
            emu.regs_mut().rax = FALSE;
            return;
        }
        strings::write_string(emu, out_username, &user_name, wide);
        log_red!(
            emu,
            "{api} returning: '{}' (size: {})",
            user_name,
            required_size
        );
        emu.regs_mut().rax = TRUE;
    }
}
