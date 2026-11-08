use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn CoTaskMemAlloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx;
    log_red!(emu, "ole32!CoTaskMemAlloc size: 0x{:x}", size);

    let addr = heap_engine::heap_allocate(emu, 0, size).unwrap_or(0);
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
