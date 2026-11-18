use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetLocaleInfoA(emu: &mut emu::Emu) {
    get_locale_info(emu, "GetLocaleInfoA", false);
}

fn get_locale_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg(emu, 0));
    let lctype = api_try!(emu, call.arg(emu, 1));
    let buf_ptr = api_try!(emu, call.arg(emu, 2));
    let buf_size = api_try!(emu, call.arg(emu, 3));
    if buf_ptr != 0 && buf_size > 0 {
        strings::write_string(emu, buf_ptr, "en-US", wide);
    }
    log_red!(
        emu,
        "kernelbase!{api} locale:0x{:x} type:0x{:x}",
        locale,
        lctype
    );

    call.set_return(emu, 6);
}

pub fn GetLocaleInfoW(emu: &mut emu::Emu) {
    get_locale_info(emu, "GetLocaleInfoW", true);
}
