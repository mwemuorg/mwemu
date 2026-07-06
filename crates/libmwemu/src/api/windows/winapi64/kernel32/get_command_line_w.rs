use crate::emu;
use crate::maps::mem64::Permission;

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCommandLineW");

    let addr = emu.maps.alloc(1024).expect("out of memory");
    let name = format!("alloc_{:x}", addr);
    emu.maps
        .create_map(&name, addr, 1024, Permission::READ_WRITE);
    let exe_name = emu.cfg.exe_name.clone();
    emu.maps.write_wide_string(addr, &exe_name);
    emu.regs_mut().rax = addr;
}
