use crate::emu;

pub fn LookupAccountSidA(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let name_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let domain_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let domain_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let use_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    log_red!(emu, "advapi32!LookupAccountSidA -> user/DESKTOP");
    let user = "user";
    let domain = "DESKTOP";
    if name_ptr != 0 {
        emu.maps.write_string(name_ptr, user);
    }
    if name_sz_ptr != 0 {
        emu.maps.write_dword(name_sz_ptr, (user.len() + 1) as u32);
    }
    if domain_ptr != 0 {
        emu.maps.write_string(domain_ptr, domain);
    }
    if domain_sz_ptr != 0 {
        emu.maps
            .write_dword(domain_sz_ptr, (domain.len() + 1) as u32);
    }
    if use_ptr != 0 {
        emu.maps.write_dword(use_ptr, 1); // SidTypeUser
    }
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn LookupAccountSidW(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let name_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let domain_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let domain_sz_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let use_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    log_red!(emu, "advapi32!LookupAccountSidW -> user/DESKTOP");
    let user = "user";
    let domain = "DESKTOP";
    if name_ptr != 0 {
        for (i, ch) in user.encode_utf16().enumerate() {
            emu.maps.write_word(name_ptr + (i as u64) * 2, ch);
        }
        emu.maps.write_word(name_ptr + (user.len() as u64) * 2, 0);
    }
    if name_sz_ptr != 0 {
        emu.maps.write_dword(name_sz_ptr, (user.len() + 1) as u32);
    }
    if domain_ptr != 0 {
        for (i, ch) in domain.encode_utf16().enumerate() {
            emu.maps.write_word(domain_ptr + (i as u64) * 2, ch);
        }
        emu.maps
            .write_word(domain_ptr + (domain.len() as u64) * 2, 0);
    }
    if domain_sz_ptr != 0 {
        emu.maps
            .write_dword(domain_sz_ptr, (domain.len() + 1) as u32);
    }
    if use_ptr != 0 {
        emu.maps.write_dword(use_ptr, 1);
    }
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
