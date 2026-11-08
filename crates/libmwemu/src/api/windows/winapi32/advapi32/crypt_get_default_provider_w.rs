use crate::emu;

pub fn CryptGetDefaultProviderW(emu: &mut emu::Emu) {
    let _prov_type = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let name_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let name = "Microsoft Base Cryptographic Provider v1.0";
    if name_ptr != 0 {
        emu.maps.write_wide_string(name_ptr, name);
    }
    if name_len_ptr != 0 {
        emu.maps
            .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
    }
    log_red!(emu, "advapi32!CryptGetDefaultProviderW => '{}'", name);
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
