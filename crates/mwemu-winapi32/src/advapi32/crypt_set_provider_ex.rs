use crate::emu;

pub fn CryptSetProviderExA(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let name = emu.maps.read_string(name_ptr);
    log_red!(
        emu,
        "advapi32!CryptSetProviderExA '{}' type:{} flags:0x{:x}",
        name,
        prov_type,
        flags
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn CryptSetProviderExW(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(
        emu,
        "advapi32!CryptSetProviderExW '{}' type:{} flags:0x{:x}",
        name,
        prov_type,
        flags
    );
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
