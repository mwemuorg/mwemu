use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;

pub(super) fn __register_frame_info(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let p1 = api_try!(emu, call.arg_u32(emu, 0));
    let p2 = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "libgcc!__register_frame_info {:x} {:x}", p1, p2);

    let mem = match emu.maps.get_mem_by_addr_mut(0x40E198) {
        Some(m) => m,
        None => match emu
            .maps
            .create_map("glob1", 0x40E198, 100, Permission::READ_WRITE_EXECUTE)
        {
            Ok(m) => m,
            Err(e) => {
                log::error!(
                    "libgcc!__register_frame_info: cannot create glob1 map: {}",
                    e
                );
                call.cleanup_args(emu, 2);
                return;
            }
        },
    };

    mem.write_dword(0x40E198, 0x6e940000);

    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}
