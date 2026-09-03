use crate::emu;

pub fn HeapDestroy(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    log_red!(emu, "kernel32!HeapDestroy hndl: 0x{:x}", hndl);

    if hndl == 0 || hndl > u32::MAX as u64 {
        emu.regs_mut().rax = 0;
        return;
    }
    let key = hndl as u32;
    if emu.handle_management.is_process_heap(key) {
        log_red!(emu, "kernel32!HeapDestroy cannot destroy process heap");
        emu.regs_mut().rax = 0;
        return;
    }
    match emu.handle_management.remove_heap_handle(key) {
        Some(_) => emu.regs_mut().rax = 1,
        None => emu.regs_mut().rax = 0,
    }
}
