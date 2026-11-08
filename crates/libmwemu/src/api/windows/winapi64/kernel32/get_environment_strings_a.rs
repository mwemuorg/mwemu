use crate::emu;
use crate::maps::mem64::Permission;

pub fn GetEnvironmentStringsA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetEnvironmentStringsA");
    let ptr = emu.alloc("environment_a", 1024, Permission::READ_WRITE);
    emu.maps.write_string(ptr, "PATH=c:\\Windows\\System32");
    emu.regs_mut().rax = ptr;
}
