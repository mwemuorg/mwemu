use crate::emu;

pub fn HeapFree(emu: &mut emu::Emu) {
    let heap = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapFree cannot read heap handle");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapFree cannot read heap handle");
    let mem = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!HeapFree cannot read heap handle");

    if emu.cfg.heap_free_soft {
        log_red!(emu, "kernel32!HeapFree mem: 0x{:x} [soft-free]", mem);
    } else {
        log_red!(emu, "kernel32!HeapFree mem: 0x{:x}", mem);
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
