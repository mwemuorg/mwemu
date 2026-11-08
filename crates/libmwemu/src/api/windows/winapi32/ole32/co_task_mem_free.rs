use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CoTaskMemFree(emu: &mut emu::Emu) {
    let pv = arg(emu, 0) as u64;
    log_red!(emu, "ole32!CoTaskMemFree pv: 0x{:x}", pv);
    if pv != 0 {
        heap_engine::heap_free(emu, 0, pv);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
