use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn LocalSize(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;

    match heap_engine::heap_allocation_size(emu, h_mem) {
        Some(size) => {
            log_red!(emu, "kernel32!LocalSize hMem:0x{:x} = {}", h_mem, size);
            emu.set_last_error(0);
            emu.regs_mut().rax = size as u64;
        }
        None => {
            log_red!(emu, "kernel32!LocalSize hMem:0x{:x} not found", h_mem);
            emu.set_last_error(6); // ERROR_INVALID_HANDLE
            emu.regs_mut().rax = 0;
        }
    }
}
