use crate::emu;
use crate::winapi::helper;

pub fn IsRectEmpty(emu: &mut emu::Emu) {
    let rect = emu.regs().rcx;
    let empty = if rect != 0 {
        let left = emu.maps.read_dword(rect).unwrap_or(0);
        let top = emu.maps.read_dword(rect + 4).unwrap_or(0);
        let right = emu.maps.read_dword(rect + 8).unwrap_or(0);
        let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0);
        right <= left || bottom <= top
    } else {
        true
    };
    log_red!(emu, "user32!IsRectEmpty 0x{:x} => {}", rect, empty);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = if empty { 1 } else { 0 };
}
