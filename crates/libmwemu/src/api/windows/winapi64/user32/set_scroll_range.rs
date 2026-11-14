use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetScrollRange(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let n_bar = emu.regs().rdx as i32;
    let n_min_pos = emu.regs().r8 as i32;
    let n_max_pos = emu.regs().r9 as i32;
    let b_redraw = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "user32!SetScrollRange hwnd:0x{:x} bar:{} min:{} max:{} redraw:{}",
        hwnd,
        n_bar,
        n_min_pos,
        n_max_pos,
        b_redraw
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
