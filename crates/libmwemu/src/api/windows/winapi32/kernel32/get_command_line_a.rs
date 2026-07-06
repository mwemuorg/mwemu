use crate::emu;
use crate::maps::mem64::Permission;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCommandlineA");

    let addr = emu.maps.alloc(1024).expect("out of memory");
    let name = format!("alloc_{:x}", addr);
    emu.maps
        .create_map(&name, addr, 1024, Permission::READ_WRITE);
    emu.maps.write_string(addr, "test.exe");
    emu.regs_mut().rax = addr;
}
