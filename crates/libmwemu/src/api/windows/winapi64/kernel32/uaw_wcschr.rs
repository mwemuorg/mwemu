use crate::emu;

pub fn uaw_wcschr(emu: &mut emu::Emu) {
    let wstr = emu.regs().rcx;
    let ch = emu.regs().rdx as u16;

    log_red!(emu, "kernel32!uaw_wcschr str:0x{:x} ch:0x{:x}", wstr, ch);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    let mut ptr = wstr;
    loop {
        let wc = emu.maps.read_word(ptr).unwrap_or(0);
        if wc == ch {
    emu.set_last_error(0);
    emu.regs_mut().rax = ptr;
    return;
        }
        if wc == 0 {
    break;
        }
        ptr += 2;
        if ptr - wstr > 0x10000 {
    break;
        }
    }
    emu.regs_mut().rax = 0;
}
