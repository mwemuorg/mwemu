use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn setvbuf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let stream = api_try!(emu, call.arg(emu, 0));
    let _buffer = api_try!(emu, call.arg_u32(emu, 1));
    let mode = api_try!(emu, call.arg_u32(emu, 2));
    let size = api_try!(emu, call.arg_u32(emu, 3));
    let uri = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!setvbuf stream:0x{:x}('{}') mode:{} size:{}",
        stream,
        uri,
        mode,
        size
    );

    emu.set_last_error(0);
    call.set_return(emu, 0); // success
}
