use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FindNextChangeNotification(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    log_red!(emu, "kernel32!FindNextChangeNotification");
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
