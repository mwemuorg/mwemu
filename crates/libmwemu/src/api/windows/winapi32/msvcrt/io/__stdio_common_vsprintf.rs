use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __stdio_common_vsprintf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let _options = api_try!(emu, call.arg_u32(emu, 0));
    let _options_hi = api_try!(emu, call.arg_u32(emu, 1));
    let buf = api_try!(emu, call.arg(emu, 2));
    let _buf_size = api_try!(emu, call.arg_u32(emu, 3));
    let fmt_ptr = api_try!(emu, call.arg(emu, 4));
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(
        emu,
        "msvcrt!__stdio_common_vsprintf buf:0x{:x} fmt:'{}'",
        buf,
        fmt
    );
    if buf != 0 && !fmt.is_empty() {
        emu.maps.write_string(buf, &fmt);
    }

    call.set_return(emu, fmt.len() as u64);
}
