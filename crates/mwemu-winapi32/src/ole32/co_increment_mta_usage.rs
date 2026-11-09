use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CoIncrementMTAUsage(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "ole32!CoIncrementMTAUsage a0:0x{:x}", _a0);
    let _h = helper::handler_create("ole32://CoIncrementMTAUsage");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
