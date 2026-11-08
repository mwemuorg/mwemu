use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn ComPs_NdrDllUnregisterProxy(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    let _a1 = arg(emu, 1);
    let _a2 = arg(emu, 2);
    log_red!(
        emu,
        "ole32!ComPs_NdrDllUnregisterProxy a0:0x{:x} a1:0x{:x}",
        _a0,
        _a1
    );
    let h = _a0 as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
