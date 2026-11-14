use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptDestroyHash(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    log_red!(emu, "advapi32!CryptDestroyHash hash: 0x{:x}", hhash);
    crypto_state::destroy_hash(hhash);
    helper::handler_close(hhash);
    emu.regs_mut().rax = 1;
}
