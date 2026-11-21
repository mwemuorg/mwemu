use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn __acrt_iob_func(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let index = api_try!(emu, call.arg_u32(emu, 0));
    let stream_name = match index {
        0 => "stdin",
        1 => "stdout",
        2 => "stderr",
        _ => "unknown_stream",
    };
    let hndl = helper::handler_create(stream_name);
    log_red!(
        emu,
        "msvcrt!__acrt_iob_func index:{} => 0x{:x}",
        index,
        hndl
    );

    call.set_return(emu, hndl);
}
