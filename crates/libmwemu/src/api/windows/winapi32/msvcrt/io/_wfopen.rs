use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _wfopen(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fname_ptr = api_try!(emu, call.arg(emu, 0));
    let mode_ptr = api_try!(emu, call.arg(emu, 1));
    let fname = emu.maps.read_wide_string(fname_ptr);
    let mode = emu.maps.read_wide_string(mode_ptr);
    let hndl = helper::handler_create(&fname);
    log_red!(
        emu,
        "msvcrt!_wfopen '{}' mode:'{}' => 0x{:x}",
        fname,
        mode,
        hndl
    );

    call.set_return(emu, hndl);
}
