use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetDeviceCaps(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let index = api_try!(emu, call.arg_i32(emu, 1));
    let result: u64 = match index {
        8 => 1920,   // HORZRES
        10 => 1080,  // VERTRES
        12 => 96,    // LOGPIXELSX
        14 => 32,    // BITSPIXEL
        88 => 96,    // LOGPIXELSX
        90 => 96,    // LOGPIXELSY
        117 => 1080, // DESKTOPVERTRES
        118 => 1920, // DESKTOPHORZRES
        _ => 0,
    };
    log_red!(emu, "gdi32!GetDeviceCaps index:{} = {}", index, result);
    emu.set_last_error(0);
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}
