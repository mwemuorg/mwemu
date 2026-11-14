use crate::emu;
use crate::maps::mem64::Permission;

pub fn __p___argv(emu: &mut emu::Emu) {
    let argv_base = match emu.maps.get_map_by_name("argv") {
        Some(a) => a.get_base(),
        None => {
            let addr = match emu.maps.alloc(1024) {
                Some(a) => a,
                None => {
                    log::error!("msvcrt!__p___argv: out of memory");
                    emu.regs_mut().rax = 0;
                    return;
                }
            };
            match emu
                .maps
                .create_map("argv", addr, 1024, Permission::READ_WRITE)
            {
                Ok(m) => m.get_base(),
                Err(e) => {
                    log::error!("msvcrt!__p___argv: cannot create argv map: {}", e);
                    emu.regs_mut().rax = 0;
                    return;
                }
            };
            let str_area = addr + 64;
            emu.maps.write_string(str_area, "mwemu.exe");
            emu.maps.write_qword(addr, str_area);
            emu.maps.write_qword(addr + 8, 0);
            addr
        }
    };
    log_red!(emu, "msvcrt!__p___argv => 0x{:x}", argv_base);
    emu.set_last_error(0);
    emu.regs_mut().rax = argv_base;
}
