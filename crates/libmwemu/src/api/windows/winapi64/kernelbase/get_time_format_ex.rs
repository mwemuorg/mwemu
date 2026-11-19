use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetTimeFormatEx(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let buffer = api_try!(emu, call.arg(emu, 4));
    let capacity = api_try!(emu, call.arg_i32(emu, 5));
    let text = "12:00:00";
    let result = strings::write_text(emu, buffer, capacity, text, true);
    log_red!(
        emu,
        "kernelbase!GetTimeFormatEx => '{}' result:{}",
        text,
        result
    );
    call.set_return(emu, result);
}
