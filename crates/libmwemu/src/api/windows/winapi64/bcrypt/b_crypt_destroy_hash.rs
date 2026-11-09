use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn BCryptDestroyHash(emu: &mut emu::Emu) {
    let h_hash = emu.regs().rcx;

    log_red!(emu, "bcrypt!BCryptDestroyHash hash=0x{:x}", h_hash);

    crypto_state::destroy_hash(h_hash);
    helper::handler_close(h_hash);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}
