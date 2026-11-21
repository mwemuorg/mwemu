use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetLocaleInfoEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale_name = api_try!(emu, call.arg(emu, 0));
    let lc_type = api_try!(emu, call.arg(emu, 1));
    let buf = api_try!(emu, call.arg(emu, 2));
    let buf_len = api_try!(emu, call.arg(emu, 3));
    let name_s = if locale_name != 0 {
        emu.maps.read_wide_string(locale_name)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!GetLocaleInfoEx '{}' type:0x{:x} buf_len:{}",
        name_s,
        lc_type,
        buf_len
    );
    if buf != 0 && buf_len > 0 {
        emu.maps.write_wide_string(buf, "en-US");
        call.set_return(emu, 6); // length of "en-US" + null
    } else {
        call.set_return(emu, 6);
    }
}
