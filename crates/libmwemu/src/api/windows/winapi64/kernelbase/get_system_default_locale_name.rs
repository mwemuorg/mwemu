use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSystemDefaultLocaleName(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let buf_ptr = api_try!(emu, call.arg(emu, 0));
    let cch_buf = api_try!(emu, call.arg(emu, 1)) as usize;
    let locale = "en-US";
    if buf_ptr != 0 {
        emu.maps.write_wide_string(buf_ptr, locale);
    }
    log_red!(emu, "kernelbase!GetSystemDefaultLocaleName => '{}'", locale);
    emu.set_last_error(0);
    call.set_return(emu, (locale.len() + 1) as u64);
}
