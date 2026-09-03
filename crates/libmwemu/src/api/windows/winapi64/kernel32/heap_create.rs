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

    let arena = emu.create_heap_arena();
    let key = emu
        .handle_management
        .insert_heap_handle(HeapHandle::new(opts, initSZ, maxSZ, arena));
    log_red!(emu, "kernel32!HeapCreate handle=0x{:x}", key as u64);
    emu.regs_mut().rax = key as u64;
}
