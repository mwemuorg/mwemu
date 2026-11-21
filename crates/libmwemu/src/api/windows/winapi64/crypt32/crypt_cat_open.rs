use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATOpen(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("CryptCATOpen");
    log_red!(
        emu,
        "crypt32!CryptCATOpen arg1:0x{:x} arg2:0x{:x} => 0x{:x}",
        arg1,
        arg2,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
