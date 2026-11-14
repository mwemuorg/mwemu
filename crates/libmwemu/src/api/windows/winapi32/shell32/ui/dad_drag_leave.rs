use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn DAD_DragLeave(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "shell32!DAD_DragLeave arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
