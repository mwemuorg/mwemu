use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn RegFlushKey(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    log_red!(emu, "advapi32!RegFlushKey");
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
