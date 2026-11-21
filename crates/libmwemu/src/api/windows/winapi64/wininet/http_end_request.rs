use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn HttpEndRequestA(emu: &mut emu::Emu) {
    http_end_request(emu, "HttpEndRequestA");
}

fn http_end_request(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "wininet!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    if arg1 != 0 && !helper::handler_exist(arg1) {
        emu.set_last_error(12019);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn HttpEndRequestW(emu: &mut emu::Emu) {
    http_end_request(emu, "HttpEndRequestW");
}
