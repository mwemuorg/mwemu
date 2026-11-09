use crate::emu;
use crate::winapi::helper;

pub fn CryptMsgClose(emu: &mut emu::Emu) {
    let msg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    helper::handler_close(msg);
    log_red!(emu, "crypt32!CryptMsgClose 0x{:x}", msg);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
