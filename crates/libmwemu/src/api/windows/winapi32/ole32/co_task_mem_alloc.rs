use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CoTaskMemAlloc(emu: &mut emu::Emu) {
    let size = arg(emu, 0) as u64;
    log_red!(emu, "ole32!CoTaskMemAlloc size: 0x{:x}", size);
    emu.regs_mut().rax = heap_engine::heap_allocate(emu, 0, size).unwrap_or(0);
    stdcall_ret(emu, 1);
}
