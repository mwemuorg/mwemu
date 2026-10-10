use crate::emu;
use crate::emu::object_handle::HeapHandle;

pub fn HeapCreate(emu: &mut emu::Emu) {
    let opts = emu.regs().rcx as u32;
    let initSZ = emu.regs().rdx;
    let maxSZ = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!HeapCreate opts: {} initSZ: {} maxSZ: {}",
        opts,
        initSZ,
        maxSZ
    );

    let arena = match emu.create_heap_arena(initSZ as usize, maxSZ as usize, opts) {
        Some(a) => a,
        None => {
            log_red!(emu, "kernel32!HeapCreate failed: cannot reserve arena");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let base = emu.heap_arenas[arena].base();
    let key = emu.handle_management.insert_heap_handle(HeapHandle::new(
        opts,
        initSZ as usize,
        maxSZ as usize,
        arena,
    ));
    emu.handle_management.set_heap_base_addr(key, base);
    log_red!(emu, "kernel32!HeapCreate handle=0x{:x}", base);
    emu.regs_mut().rax = base;
}
