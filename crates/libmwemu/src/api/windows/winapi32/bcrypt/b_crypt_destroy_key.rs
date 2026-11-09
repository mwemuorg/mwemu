use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants;

pub(super) fn BCryptDestroyKey(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    log_red!(emu, "bcrypt!BCryptDestroyKey key=0x{:x}", h_key);
    crypto_state::destroy_key(h_key);
    helper::handler_close(h_key);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 1);
}
