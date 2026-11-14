use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn BitBlt(emu: &mut emu::Emu) {
    let hdc_dest = emu.regs().rcx;
    let x_dest = emu.regs().rdx as i32;
    let y_dest = emu.regs().r8 as i32;
    let width = emu.regs().r9 as i32;
    let height = WinApiAbi::arg(emu, 4) as i32;
    let hdc_src = WinApiAbi::arg(emu, 5);
    let x_src = WinApiAbi::arg(emu, 6) as i32;
    let y_src = WinApiAbi::arg(emu, 7) as i32;
    let rop = WinApiAbi::arg(emu, 8) as u32;
    log_red!(
        emu,
        "gdi32!BitBlt dst:0x{:x} x:{} y:{} {}x{} src:0x{:x} x:{} y:{} rop:0x{:x}",
        hdc_dest,
        x_dest,
        y_dest,
        width,
        height,
        hdc_src,
        x_src,
        y_src,
        rop
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
