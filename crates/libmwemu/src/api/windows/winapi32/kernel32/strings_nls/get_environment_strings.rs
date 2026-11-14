use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper::WinApiAbi;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetEnvironmentStrings");
    let ptr = emu.alloc("environment", 1024, Permission::READ_WRITE);
    emu.maps.write_string(ptr, "PATH=c:\\Windows\\System32");
    emu.regs_mut().rax = ptr;
}

pub fn GetEnvironmentStringsA(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "kernel32!GetEnvironmentStringsA arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetEnvironmentStringsW");
    let addr = emu.alloc("environment", 1024, Permission::READ_WRITE);
    emu.maps
        .write_wide_string(addr, "PATH=c:\\Windows\\System32");
    emu.regs_mut().rax = addr;
}
