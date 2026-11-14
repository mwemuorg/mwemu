use crate::emu;
use crate::winapi::helper;

pub fn CryptDestroyKey(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;
    log_red!(emu, "crypt32!CryptDestroyKey key: 0x{:x}", h_key);
    if h_key != 0 {
        helper::handler_close(h_key);
    }
    emu.regs_mut().rax = 1; // TRUE
}
