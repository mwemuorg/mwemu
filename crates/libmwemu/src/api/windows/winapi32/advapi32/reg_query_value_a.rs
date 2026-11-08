use crate::emu;

pub fn RegQueryValueA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let name = emu.maps.read_string(name_ptr);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    log_red!(emu, "advapi32!RegQueryValueA hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 2;
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
