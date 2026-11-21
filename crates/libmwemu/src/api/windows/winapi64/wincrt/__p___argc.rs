use super::allocate_memory;
use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn __p___argc(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let argc = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "wincrt!__p___argc argc: 0x{:x}", argc);

    let argc_addr = match allocate_memory(emu, 4) {
        Some(a) => a,
        None => {
            log::error!("wincrt!__p___argc cannot allocate");
            call.set_return(emu, 0);
            return;
        }
    };
    emu.maps.write_dword(argc_addr, 1);
    call.set_return(emu, argc_addr);
}
