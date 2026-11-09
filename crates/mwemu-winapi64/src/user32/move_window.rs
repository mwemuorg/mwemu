use crate::emu;

pub fn MoveWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let width = emu.regs().r9 as i32;
    let height = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as i32;
    let repaint = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "user32!MoveWindow hwnd:0x{:x} x:{} y:{} w:{} h:{} repaint:{}",
        hwnd,
        x,
        y,
        width,
        height,
        repaint
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
