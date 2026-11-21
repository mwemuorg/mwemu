use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _wopen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let oflag = api_try!(emu, call.arg_u32(emu, 1));
    let path = emu.maps.read_wide_string(path_ptr);
    let hndl = helper::handler_create(&path);
    log_red!(
        emu,
        "wincrt!_wopen '{}' flags:0x{:x} => 0x{:x}",
        path,
        oflag,
        hndl
    );
    call.set_return(emu, hndl);
}
