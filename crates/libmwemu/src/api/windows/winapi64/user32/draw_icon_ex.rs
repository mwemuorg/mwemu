use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn DrawIconEx(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x_left = emu.regs().rdx as i32;
    let y_top = emu.regs().r8 as i32;
    let h_icon = emu.regs().r9;
    let cx_width = WinApiAbi::arg(emu, 4) as i32;
    let cy_width = WinApiAbi::arg(emu, 5) as i32;
    let _ist_step_if_ani_cur = WinApiAbi::arg(emu, 6);
    let _hbr_flicker_free_draw = WinApiAbi::arg(emu, 7);
    let _di_flags = WinApiAbi::arg(emu, 8);
    log_red!(
        emu,
        "user32!DrawIconEx hdc:0x{:x} x:{} y:{} icon:0x{:x} cx:{} cy:{}",
        hdc,
        x_left,
        y_top,
        h_icon,
        cx_width,
        cy_width
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
