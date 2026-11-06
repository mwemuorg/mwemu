
use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::helper;
pub fn GlobalReAlloc(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;
    let dw_bytes = emu.regs().rdx;
    let u_flags = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GlobalReAlloc hMem:0x{:x} bytes:{} flags:0x{:x}",
        h_mem,
        dw_bytes,
        u_flags
    );
    let handle = helper::handler_create("GlobalReAlloc");
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    // If size is 0, free and return 0
    if dw_bytes == 0 {
        if h_mem != 0 {
    heap_engine::heap_free(emu, 0, h_mem);
        }
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
        return;
    }
    // If hMem is 0, allocate new
    if h_mem == 0 {
        match heap_engine::heap_allocate(emu, 0, dw_bytes) {
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
    match heap_engine::heap_reallocate(emu, 0, h_mem, dw_bytes) {
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
