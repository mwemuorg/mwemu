use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CoGetCurrentLogicalThreadId(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "ole32!CoGetCurrentLogicalThreadId a0:0x{:x}", _a0);
    let _h = helper::handler_create("ole32://CoGetCurrentLogicalThreadId");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
