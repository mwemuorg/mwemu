use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn StretchBlt(emu: &mut emu::Emu) {
    let hdc_dest = emu.regs().rcx;
    let x_dest = emu.regs().rdx as i32;
    let y_dest = emu.regs().r8 as i32;
    let w_dest = emu.regs().r9 as i32;
    let h_dest = WinApiAbi::arg(emu, 4) as i32;
    let hdc_src = WinApiAbi::arg(emu, 5);
    let x_src = WinApiAbi::arg(emu, 6) as i32;
    let y_src = WinApiAbi::arg(emu, 7) as i32;
    let w_src = WinApiAbi::arg(emu, 8) as i32;
    let h_src = WinApiAbi::arg(emu, 9) as i32;
    let rop = WinApiAbi::arg(emu, 10) as u32;
    log_red!(
        emu,
        "gdi32!StretchBlt dst:0x{:x} x:{} y:{} {}x{} src:0x{:x} x:{} y:{} {}x{} rop:0x{:x}",
        hdc_dest,
        x_dest,
        y_dest,
        w_dest,
        h_dest,
        hdc_src,
        x_src,
        y_src,
        w_src,
        h_src,
        rop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
