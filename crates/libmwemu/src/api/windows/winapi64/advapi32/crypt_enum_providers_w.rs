use crate::emu;

pub fn CryptEnumProvidersW(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let prov_type_ptr = emu.regs().r9;
    let name_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let name_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
}
