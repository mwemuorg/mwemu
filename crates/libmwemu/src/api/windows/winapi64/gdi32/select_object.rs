use crate::emu;
use crate::winapi::helper;

pub fn SelectObject(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let obj = emu.regs().rdx;
    log_red!(emu, "gdi32!SelectObject hdc:0x{:x} obj:0x{:x}", hdc, obj);
    if !helper::handler_exist(hdc) {
        log_red!(emu, "gdi32!SelectObject invalid hdc");
        emu.regs_mut().rax = 0;
        return;
    }
    if !helper::handler_exist(obj) {
        log_red!(emu, "gdi32!SelectObject invalid obj");
        emu.regs_mut().rax = 0;
        return;
    }
    // Return previous object (simulate by returning the obj being selected)
    emu.regs_mut().rax = obj;
}
