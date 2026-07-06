use crate::emu;
use crate::utils::helpers::likely;

pub fn HeapFree(emu: &mut emu::Emu) {
    let heap = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mem = emu.regs().r8;

    if emu.cfg.heap_free_soft {
        // Soft-free: mark as freed but don't actually deallocate memory
        log_red!(emu, "kernel32!HeapFree mem: 0x{:x} [soft-free]", mem);
    } else {
        let heap_manage = emu.heap_mut();
        if likely(heap_manage.check_fragment_exists(mem)) {
            heap_manage.free(mem);
        }
        heap_manage.free(mem);
        emu.maps.dealloc(mem);
        log_red!(emu, "kernel32!HeapFree mem: 0x{:x}", mem);
    }

    emu.regs_mut().rax = 1;
}
