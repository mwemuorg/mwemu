use crate::api::windows::common::heap as heap_engine;
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

    let arena = emu.create_heap_arena(initSZ as usize, maxSZ as usize, opts);
    let key = emu.handle_management.insert_heap_handle(HeapHandle::new(
        opts,
        initSZ as usize,
        maxSZ as usize,
        arena,
    ));
    let addr = heap_engine::heap_handle_address(emu, key);
    log_red!(emu, "kernel32!HeapCreate handle=0x{:x}", addr);
    emu.regs_mut().rax = addr;
}
