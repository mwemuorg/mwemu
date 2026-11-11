use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptDestroyHash(emu: &mut emu::Emu) {
    let h_hash = arg(emu, 0) as u64;
    log_red!(emu, "bcrypt!BCryptDestroyHash hash=0x{:x}", h_hash);
    crypto_state::destroy_hash(h_hash);
    helper::handler_close(h_hash);
    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 1);
}
