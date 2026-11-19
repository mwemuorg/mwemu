use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn fopen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let filepath_ptr = api_try!(emu, call.arg(emu, 0));
    let mode_ptr = api_try!(emu, call.arg(emu, 1));

    if filepath_ptr == 0 || mode_ptr == 0 {
        call.set_return(emu, 0xffffffff);
        log_red!(emu, "msvcrt!fopen null ptrs");
        return;
    }

    let filepath = emu.maps.read_string(filepath_ptr);
    let mode = emu.maps.read_string(mode_ptr);

    log_red!(emu, "msvcrt!fopen `{}` fmt:`{}`", filepath, mode);

    call.set_return(emu, helper::handler_create(&filepath));
}
