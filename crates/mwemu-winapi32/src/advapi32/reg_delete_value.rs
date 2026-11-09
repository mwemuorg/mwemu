use crate::emu;

pub fn RegDeleteValueA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "advapi32!RegDeleteValueA hkey:0x{:x} '{}'", hkey, name);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteValueW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "advapi32!RegDeleteValueW hkey:0x{:x} '{}'", hkey, name);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
