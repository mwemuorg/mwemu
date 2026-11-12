use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSACancelBlockingCall(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "ws2_32!WSACancelBlockingCall 0x{:x}", arg1);
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
