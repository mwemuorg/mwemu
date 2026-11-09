use crate::emu;
use crate::winapi::helper;

pub fn CryptDestroyHash(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;
    log_red!(emu, "crypt32!CryptDestroyHash hash: 0x{:x}", h_hash);
    if h_hash != 0 {
        helper::handler_close(h_hash);
    }
    emu.regs_mut().rax = 1; // TRUE
}
