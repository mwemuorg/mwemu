use crate::emu;

pub fn LineTo(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    log_red!(emu, "gdi32!LineTo hdc:0x{:x} x:{} y:{}", hdc, x, y);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
