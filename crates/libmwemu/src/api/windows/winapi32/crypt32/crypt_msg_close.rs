use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptMsgClose(emu: &mut emu::Emu) {
    let msg = WinApiAbi::arg(emu, 0);
    helper::handler_close(msg);
    log_red!(emu, "crypt32!CryptMsgClose 0x{:x}", msg);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
