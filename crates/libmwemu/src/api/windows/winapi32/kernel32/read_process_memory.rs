use crate::emu;

pub fn ReadProcessMemory(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!ReadProcessMemory cannot read the handle");
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!ReadProcessMemory cannot read the base address");
    let buff = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!ReadProcessMemory cannot read buff") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!ReadProcessMemory cannot read size");
    let bytes = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!ReadProcessMemory cannot read bytes") as u64;

    log_red!(
        emu,
        "kernel32!ReadProcessMemory hndl: {} from: 0x{:x} to: 0x{:x} sz: {}",
        hndl,
        addr,
        buff,
        size
    );

    emu.maps.write_dword(bytes, size);
    emu.maps.memset(buff, 0x90, size as usize);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
