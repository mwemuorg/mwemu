use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn BCryptDestroyKey(emu: &mut emu::Emu) {
    let h_key = emu.regs().rcx;

    log_red!(emu, "bcrypt!BCryptDestroyKey key=0x{:x}", h_key);

    crypto_state::destroy_key(h_key);
    helper::handler_close(h_key);
    emu.regs_mut().rax = STATUS_SUCCESS;
}
