use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn DeleteService(emu: &mut emu::Emu) {
    let hsvc = WinApiAbi::arg(emu, 0);
    helper::handler_close(hsvc);
    log_red!(emu, "advapi32!DeleteService 0x{:x}", hsvc);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
