use crate::emu;

pub fn CryptEnumProvidersW(emu: &mut emu::Emu) {
    let index = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let prov_type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let name_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    log_red!(emu, "advapi32!CryptEnumProvidersW index:{}", index);
    if index == 0 {
        if prov_type_ptr != 0 {
            emu.maps.write_dword(prov_type_ptr, 1);
        }
        let name = "Microsoft Base Cryptographic Provider v1.0";
        if name_ptr != 0 {
            emu.maps.write_wide_string(name_ptr, name);
        }
        if name_len_ptr != 0 {
            emu.maps
                .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
        }
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
}
