use crate::emu;

pub fn LookupSecurityDescriptorPartsA(emu: &mut emu::Emu) {
    let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "advapi32!LookupSecurityDescriptorPartsA '{}'", name);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn LookupSecurityDescriptorPartsW(emu: &mut emu::Emu) {
    let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = emu.maps.read_wide_string(name_ptr);
    log_red!(emu, "advapi32!LookupSecurityDescriptorPartsW '{}'", name);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
