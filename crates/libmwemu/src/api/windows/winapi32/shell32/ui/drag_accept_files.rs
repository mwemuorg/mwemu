use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn DragAcceptFiles(emu: &mut emu::Emu) {
    let hwnd = WinApiAbi::arg(emu, 0);
    let accept = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "shell32!DragAcceptFiles hwnd:0x{:x} accept:{}",
        hwnd,
        accept
    );
    emu.set_last_error(0);
    let h = WinApiAbi::arg(emu, 0);
    emu.regs_mut().rax = 1;

    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
