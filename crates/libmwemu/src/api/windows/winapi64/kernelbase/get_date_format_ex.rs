use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetDateFormatEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _locale_ptr = api_try!(emu, call.arg(emu, 0));
    let _flags = api_try!(emu, call.arg(emu, 1));
    let _date_ptr = api_try!(emu, call.arg(emu, 2));
    let _format_ptr = api_try!(emu, call.arg(emu, 3));
    let buf_ptr = api_try!(emu, call.arg(emu, 4));
    let buf_size = api_try!(emu, call.arg_i32(emu, 5));
    let date = "11/01/2026";
    let written = strings::write_text(emu, buf_ptr, buf_size, date, true);
    log_red!(emu, "kernelbase!GetDateFormatEx => '{}'", date);

    call.set_return(emu, written);
}
