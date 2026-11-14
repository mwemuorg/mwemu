use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn CoInitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoInitialize");
    emu.regs_mut().rax = S_OK;
    stdcall_ret(emu, 1);
}
