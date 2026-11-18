use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::maps::mem64::Permission;

pub(super) fn __p___argv(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let args_base = match emu.maps.get_map_by_name("args") {
        Some(a) => a.get_base(),
        None => {
            let addr = match emu.maps.alloc(1024) {
                Some(a) => a,
                None => {
                    log::error!("msvcrt!__p___argv: out of memory");
                    call.set_return(emu, 0);
                    return;
                }
            };
            match emu
                .maps
                .create_map("args", addr, 1024, Permission::READ_WRITE)
            {
                Ok(m) => m.get_base(),
                Err(e) => {
                    log::error!("msvcrt!__p___argv: cannot create args map: {}", e);
                    call.set_return(emu, 0);
                    return;
                }
            }
        }
    };
    log_red!(emu, "msvcrt!__p___argv => 0x{:x}", args_base + 4);
    call.set_return(emu, args_base + 4);
}
