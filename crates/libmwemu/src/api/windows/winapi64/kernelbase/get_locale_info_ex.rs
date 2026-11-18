use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetLocaleInfoEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale_ptr = api_try!(emu, call.arg(emu, 0));
    let lctype = api_try!(emu, call.arg(emu, 1));
    let buf_ptr = api_try!(emu, call.arg(emu, 2));
    let buf_size = api_try!(emu, call.arg(emu, 3));
    let locale = emu.maps.read_wide_string(locale_ptr);
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, "en-US");
    }
    log_red!(
        emu,
        "kernelbase!GetLocaleInfoEx '{}' type:0x{:x}",
        locale,
        lctype
    );
    call.set_return(emu, 6);
}
