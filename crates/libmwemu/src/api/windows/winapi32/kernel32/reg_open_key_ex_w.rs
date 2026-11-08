use crate::emu;

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let options = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let sam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let result = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let subkey = emu.maps.read_wide_string(subkey_ptr);
    log_red!(emu, "kernel32!RegOpenKeyExW {}", subkey);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
