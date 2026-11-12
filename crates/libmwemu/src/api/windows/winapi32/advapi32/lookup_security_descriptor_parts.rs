use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn LookupSecurityDescriptorPartsA(emu: &mut emu::Emu) {
    lookup_security_descriptor_parts(emu, "LookupSecurityDescriptorPartsA", false);
}

fn lookup_security_descriptor_parts(emu: &mut emu::Emu, api: &str, wide: bool) {
    let system_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}

pub fn LookupSecurityDescriptorPartsW(emu: &mut emu::Emu) {
    lookup_security_descriptor_parts(emu, "LookupSecurityDescriptorPartsW", true);
}
