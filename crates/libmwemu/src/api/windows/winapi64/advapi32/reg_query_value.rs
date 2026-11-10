use crate::emu;

pub fn RegQueryValueA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let name = emu.maps.read_string(name_ptr);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    log_red!(emu, "advapi32!RegQueryValueA hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 2;
}

pub fn RegQueryValueW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let name = emu.maps.read_wide_string(name_ptr);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    log_red!(emu, "advapi32!RegQueryValueW hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 2;
}
