use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GopherGetLocatorTypeA(emu: &mut emu::Emu) {
    gopher_get_locator_type(emu, "GopherGetLocatorTypeA");
}

fn gopher_get_locator_type(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    call.set_return(emu, 0);
}

pub fn GopherGetLocatorTypeW(emu: &mut emu::Emu) {
    gopher_get_locator_type(emu, "GopherGetLocatorTypeW");
}
