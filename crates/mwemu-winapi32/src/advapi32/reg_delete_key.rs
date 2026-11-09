use crate::emu;

pub fn RegDeleteKeyA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let subkey = emu.maps.read_string(subkey_ptr);
    log_red!(emu, "advapi32!RegDeleteKeyA hkey:0x{:x} '{}'", hkey, subkey);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteKeyW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let subkey = emu.maps.read_wide_string(subkey_ptr);
    log_red!(emu, "advapi32!RegDeleteKeyW hkey:0x{:x} '{}'", hkey, subkey);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
