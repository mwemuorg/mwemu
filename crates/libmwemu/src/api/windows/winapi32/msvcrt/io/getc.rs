use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn getc(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let stream = api_try!(emu, call.arg(emu, 0));
    let name = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!{} stream:0x{:x} '{}' => EOF",
        api,
        stream,
        name
    );
    emu.set_last_error(0);
    call.set_return(emu, 0xffff_ffff); // EOF
}
