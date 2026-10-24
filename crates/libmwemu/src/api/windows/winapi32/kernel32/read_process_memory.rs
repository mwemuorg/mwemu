use crate::emu;

pub fn ReadProcessMemory(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let addr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let buff = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let bytes = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

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
