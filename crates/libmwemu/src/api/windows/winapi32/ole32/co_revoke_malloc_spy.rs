use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CoRevokeMallocSpy(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "ole32!CoRevokeMallocSpy a0:0x{:x}", _a0);
    let h = _a0 as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
