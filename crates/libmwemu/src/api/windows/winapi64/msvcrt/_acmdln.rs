use crate::emu;
use crate::maps::mem64::Permission;

pub fn _acmdln(emu: &mut emu::Emu) {
    let args_base = match emu.maps.get_map_by_name("args") {
        Some(a) => a.get_base(),
        None => {
            let addr = match emu.maps.alloc(1024) {
                Some(a) => a,
                None => {
                    log::error!("msvcrt!__p__acmdln: out of memory");
                    emu.regs_mut().rax = 0;
                    return;
                }
            };
            match emu
                .maps
                .create_map("args", addr, 1024, Permission::READ_WRITE)
            {
                Ok(m) => m.get_base(),
                Err(e) => {
                    log::error!("msvcrt!__p__acmdln: cannot create args map: {}", e);
                    emu.regs_mut().rax = 0;
                    return;
                }
            };
            let str_area = addr + 128;
            emu.maps.write_string(str_area, "mwemu.exe");
            emu.maps.write_qword(addr + 16, str_area);
            addr
        }
    };
    log_red!(emu, "msvcrt!__p__acmdln => 0x{:x}", args_base + 16);
    emu.set_last_error(0);
    emu.regs_mut().rax = args_base + 16;
}
