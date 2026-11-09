use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn BCryptFinalizeKeyPair(emu: &mut emu::Emu) {
    let h_key = arg(emu, 0) as u64;
    let _flags = arg(emu, 1);

    log_red!(emu, "bcrypt!BCryptFinalizeKeyPair key=0x{:x}", h_key);

    emu.regs_mut().rax = STATUS_SUCCESS;
    stdcall_ret(emu, 2);
}
