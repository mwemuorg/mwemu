use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants;

pub(super) fn BCryptCloseAlgorithmProvider(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(
        emu,
        "bcrypt!BCryptCloseAlgorithmProvider handle: 0x{:x}",
        handle
    );
    helper::handler_close(handle);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
    stdcall_ret(emu, 2);
}
