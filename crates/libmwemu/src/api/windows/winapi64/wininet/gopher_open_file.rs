use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GopherOpenFileA(emu: &mut emu::Emu) {
    gopher_open_file(emu, "GopherOpenFileA");
}

fn gopher_open_file(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    let arg5 = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "wininet!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let handle = helper::handler_create(api);

    emu.set_last_error(2);
    call.set_return(emu, 0xFFFFFFFFFFFFFFFF);
}

pub fn GopherOpenFileW(emu: &mut emu::Emu) {
    gopher_open_file(emu, "GopherOpenFileW");
}
