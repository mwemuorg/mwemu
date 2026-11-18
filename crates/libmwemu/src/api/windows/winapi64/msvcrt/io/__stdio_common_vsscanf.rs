use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __stdio_common_vsscanf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let _options = api_try!(emu, call.arg(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let _buf_size = api_try!(emu, call.arg(emu, 2));
    let fmt_ptr = api_try!(emu, call.arg(emu, 3));
    let s = emu.maps.read_string(buf);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(
        emu,
        "msvcrt!__stdio_common_vsscanf buf:'{}' fmt:'{}'",
        s,
        fmt
    );
    call.set_return(emu, 1);
}
