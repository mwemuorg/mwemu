use crate::emu;

pub fn HeapDestroy(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapDestroy cannot read handle") as u64;

    log_red!(emu, "kernel32!HeapDestroy hndl: 0x{:x}", hndl);

    emu.stack_pop32(false);

    let key = match emu.handle_management.resolve_heap_key(hndl) {
        Some(k) => k,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    if emu.handle_management.is_process_heap(key) {
        log_red!(emu, "kernel32!HeapDestroy cannot destroy process heap");
        emu.regs_mut().rax = 0;
        return;
    }
    match emu.handle_management.remove_heap_handle(key) {
        Some(handle) => {
            emu.destroy_heap_arena(handle.arena);
            emu.regs_mut().rax = 1;
        }
        None => emu.regs_mut().rax = 0,
    }
}
