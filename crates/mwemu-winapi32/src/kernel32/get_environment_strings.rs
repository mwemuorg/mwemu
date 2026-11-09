use crate::emu;
use crate::maps::mem64::Permission;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetEnvironmentStrings");
    let ptr = emu.alloc("environment", 1024, Permission::READ_WRITE);
    emu.maps.write_string(ptr, "PATH=c:\\Windows\\System32");
    emu.regs_mut().rax = ptr;
}
