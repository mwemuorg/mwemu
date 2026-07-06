use crate::emu;

pub fn VirtualFree(emu: &mut emu::Emu) {
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!VirtualFree cannot read addr") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!VirtualFree cannot read out_buff");
    let freeType = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!VirtualFree cannot read size") as u64;

    log_red!(emu, "kernel32!VirtualFree 0x{:x} {}", addr, size);

    match emu.maps.get_mem_by_addr(addr) {
        Some(mem) => {
            emu.maps.dealloc(mem.get_base());
            emu.regs_mut().rax = 1;
        }
        None => {
            emu.regs_mut().rax = 0;
        }
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
