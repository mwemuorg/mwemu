use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn ExtTextOutA(emu: &mut emu::Emu) {
    ext_text_out(emu, "ExtTextOutA", false);
}

fn ext_text_out(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let x = api_try!(emu, call.arg_i32(emu, 1));
    let y = api_try!(emu, call.arg_i32(emu, 2));
    let options = api_try!(emu, call.arg_u32(emu, 3));
    let _lprect = api_try!(emu, call.arg(emu, 4));
    let text_ptr = api_try!(emu, call.arg(emu, 5));
    let count = api_try!(emu, call.arg_u32(emu, 6));
    let text = if text_ptr != 0 && count > 0 {
        strings::read_text(emu, text_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} x:{} y:{} options:0x{:x} count:{} text:'{}'",
        hdc,
        x,
        y,
        options,
        count,
        text
    );
    call.set_return(emu, 1);
}

pub fn ExtTextOutW(emu: &mut emu::Emu) {
    ext_text_out(emu, "ExtTextOutW", true);
}
