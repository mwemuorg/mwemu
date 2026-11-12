use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn ScrollWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let x_amount = emu.regs().rdx as i32;
    let y_amount = emu.regs().r8 as i32;
    let lp_rect = emu.regs().r9;
    let lp_clip_rect = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "user32!ScrollWindow hwnd:0x{:x} dx:{} dy:{} rect:0x{:x} clip:0x{:x}",
        hwnd,
        x_amount,
        y_amount,
        lp_rect,
        lp_clip_rect
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
