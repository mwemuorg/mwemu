use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn PatBlt(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let w = emu.regs().r9 as i32;
    let h = WinApiAbi::arg(emu, 4) as i32;
    let rop = WinApiAbi::arg(emu, 5) as u32;
    log_red!(
        emu,
        "gdi32!PatBlt hdc:0x{:x} x:{} y:{} w:{} h:{} rop:0x{:x}",
        hdc,
        x,
        y,
        w,
        h,
        rop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
