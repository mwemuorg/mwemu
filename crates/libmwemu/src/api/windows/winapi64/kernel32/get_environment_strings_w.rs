use crate::emu;
use crate::maps::mem64::Permission;

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetEnvironmentStringsW");
    let addr = emu.alloc("environment", 1024, Permission::READ_WRITE);
    emu.maps
        .write_wide_string(addr, "PATH=c:\\Windows\\System32");
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
