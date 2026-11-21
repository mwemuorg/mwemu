use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _set_app_type(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "msvcrt!_set_app_type arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );

    emu.set_last_error(0);
    call.set_return(emu, 1);
}
