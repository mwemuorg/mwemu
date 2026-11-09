use crate::emu;

pub fn RegSetKeyValueW(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let reg_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(
        emu,
        "advapi32!RegSetKeyValueW hkey:0x{:x} '{}' type:{}",
        hkey,
        name,
        reg_type
    );
    emu.regs_mut().rax = 0;
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
