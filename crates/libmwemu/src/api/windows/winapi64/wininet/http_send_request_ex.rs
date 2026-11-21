use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn HttpSendRequestExA(emu: &mut emu::Emu) {
    http_send_request_ex(emu, "HttpSendRequestExA");
}

fn http_send_request_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let buffers_in = api_try!(emu, call.arg(emu, 1));
    let buffers_out = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} buf_in:0x{:x} flags:0x{:x}",
        handle,
        buffers_in,
        flags
    );

    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn HttpSendRequestExW(emu: &mut emu::Emu) {
    http_send_request_ex(emu, "HttpSendRequestExW");
}
