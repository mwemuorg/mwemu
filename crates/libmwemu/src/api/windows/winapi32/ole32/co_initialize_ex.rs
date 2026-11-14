use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub(super) fn CoInitializeEx(emu: &mut emu::Emu) {
    let co_init = arg(emu, 1);
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    emu.regs_mut().rax = S_OK;
    stdcall_ret(emu, 2);
}
