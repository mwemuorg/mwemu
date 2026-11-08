use crate::emu;

pub fn LookupPrivilegeDisplayNameA(emu: &mut emu::Emu) {
    let system_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "advapi32!LookupPrivilegeDisplayNameA '{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
