use crate::emu;

pub fn LookupAccountSidA(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().r8;
    let name_sz_ptr = emu.regs().r9;
    let domain_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let domain_sz_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let use_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
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
    emu.regs_mut().rax = 1;
}
