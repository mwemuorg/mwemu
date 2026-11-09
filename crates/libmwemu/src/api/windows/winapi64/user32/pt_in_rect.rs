use crate::emu;

pub fn PtInRect(emu: &mut emu::Emu) {
    let rect = emu.regs().rcx;
    let pt_x = emu.regs().rdx as i32;
    let pt_y = emu.regs().r8 as i32;
    let inside = if rect != 0 {
        let left = emu.maps.read_dword(rect).unwrap_or(0) as i32;
        let top = emu.maps.read_dword(rect + 4).unwrap_or(0) as i32;
        let right = emu.maps.read_dword(rect + 8).unwrap_or(0) as i32;
        let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0) as i32;
        pt_x >= left && pt_x < right && pt_y >= top && pt_y < bottom
    } else {
        false
    };
    log_red!(
        emu,
        "user32!PtInRect rect:0x{:x} pt:({},{}) => {}",
        rect,
        pt_x,
        pt_y,
        inside
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = if inside { 1 } else { 0 };
}
