use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::helper;
pub fn LocalReAlloc(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;
    let new_size = emu.regs().rdx;
    let flags = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!LocalReAlloc hMem:0x{:x} size:{} flags:0x{:x}",
        h_mem,
        new_size,
        flags
    );
    let handle = helper::handler_create("LocalReAlloc");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    // If size is 0, free and return 0
    if new_size == 0 {
        if h_mem != 0 {
            heap_engine::heap_free(emu, 0, h_mem);
        }
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }
    // If hMem is 0, allocate new
    if h_mem == 0 {
        match heap_engine::heap_allocate(emu, 0, new_size) {
            Some(addr) => {
                emu.set_last_error(0);
                emu.regs_mut().rax = addr;
            }
            None => {
                emu.set_last_error(0x0000000E); // ERROR_OUTOFMEMORY
                emu.regs_mut().rax = 0;
            }
        }
        return;
    }
    // Reallocate existing block
    match heap_engine::heap_reallocate(emu, 0, h_mem, new_size) {
        Some(new_addr) => {
            emu.set_last_error(0);
            emu.regs_mut().rax = new_addr;
        }
        None => {
            emu.set_last_error(0x0000000E); // ERROR_OUTOFMEMORY
            emu.regs_mut().rax = 0;
        }
    }
}
