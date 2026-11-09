use crate::emu;

pub fn RegEnumKeyExA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!RegEnumKeyExA hkey:0x{:x} idx:{}",
        hkey,
        index
    );
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}

pub fn RegEnumKeyExW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let index = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!RegEnumKeyExW hkey:0x{:x} idx:{}",
        hkey,
        index
    );
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 259;
}
