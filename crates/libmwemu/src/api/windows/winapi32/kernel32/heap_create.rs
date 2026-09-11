use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::emu::object_handle::HeapHandle;

pub fn HeapCreate(emu: &mut emu::Emu) {
    let opts = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapCreate cannot read opts");
    let init_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapCreate cannot read init_sz");
    let max_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!HeapCreate cannot read max_sz");

    log_red!(
        emu,
        "kernel32!HeapCreate opts: {} initSz: {} maxSz: {}",
        opts,
        init_sz,
        max_sz
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let arena = emu.create_heap_arena(init_sz as usize, max_sz as usize, opts);
    let key = emu.handle_management.insert_heap_handle(HeapHandle::new(
        opts,
        init_sz as usize,
        max_sz as usize,
        arena,
    ));
    let addr = heap_engine::heap_handle_address(emu, key);
    log_red!(emu, "kernel32!HeapCreate handle=0x{:x}", addr);
    emu.regs_mut().rax = addr;
}
