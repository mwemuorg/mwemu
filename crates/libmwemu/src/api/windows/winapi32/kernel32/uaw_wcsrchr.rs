use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn uaw_wcsrchr(emu: &mut emu::Emu) {
    let wstr = arg(emu, 0) as u64;
    let ch = arg(emu, 1) as u64 as u16;

    log_red!(emu, "kernel32!uaw_wcsrchr str:0x{:x} ch:0x{:x}", wstr, ch);
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    let mut ptr = wstr;
    let mut last_found: u64 = 0;
    loop {
        let wc = emu.maps.read_word(ptr).unwrap_or(0);
        if wc == ch {
            last_found = ptr;
        }
        if wc == 0 {
            break;
        }
        ptr += 2;
        if ptr - wstr > 0x10000 {
            break;
        }
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = last_found;
    stdcall_ret(emu, 2);
}
