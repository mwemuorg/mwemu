use crate::emu;
use crate::maps::mem64::Permission;

pub(super) fn __p___argc(emu: &mut emu::Emu) {
    let args_base = match emu.maps.get_map_by_name("args") {
        Some(a) => a.get_base(),
        None => {
            let addr = match emu.maps.alloc(1024) {
                Some(a) => a,
                None => {
                    log::error!("msvcrt!__p___argc: out of memory");
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
                    log::error!("msvcrt!__p___argc: cannot create args map: {}", e);
                    emu.regs_mut().rax = 0;
                    return;
                }
            }
        }
    };

    log_red!(emu, "msvcrt!__p___argc {}", args_base);

    emu.regs_mut().rax = args_base;
}
