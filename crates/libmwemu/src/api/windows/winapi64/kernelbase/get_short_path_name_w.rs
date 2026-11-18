use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetShortPathNameW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let long_ptr = api_try!(emu, call.arg(emu, 0));
    let short_ptr = api_try!(emu, call.arg(emu, 1));
    let buf_size = api_try!(emu, call.arg(emu, 2));
    let path = if long_ptr != 0 {
        emu.maps.read_wide_string(long_ptr)
    } else {
        String::new()
    };
    let char_count = path.encode_utf16().count() + 1;
    if short_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(short_ptr, &path);
    }
    log_red!(
        emu,
        "kernelbase!GetShortPathNameW '{}' => len:{}",
        path,
        char_count
    );
    emu.set_last_error(0);
    call.set_return(emu, char_count as u64);
}
