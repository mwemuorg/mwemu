use crate::emu;

pub fn DragAcceptFiles(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let accept = emu.regs().rdx as u32;
    log_red!(
        emu,
        "shell32!DragAcceptFiles hwnd:0x{:x} accept:{}",
        hwnd,
        accept
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
