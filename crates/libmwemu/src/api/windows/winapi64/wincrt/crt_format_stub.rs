use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn crt_format_stub(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    match api {
        "sprintf" | "snprintf" | "_snprintf" | "_sprintf_l" => {
            let buf = api_try!(emu, call.arg(emu, 0));
            let fmt_ptr = api_try!(emu, call.arg(emu, 1));
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "wincrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
                call.set_return(emu, fmt.len() as u64);
            } else {
                call.set_return(emu, 0);
            }
        }
        "fprintf" => {
            let stream = api_try!(emu, call.arg(emu, 0));
            let fmt_ptr = api_try!(emu, call.arg(emu, 1));
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "wincrt!fprintf stream:0x{:x} fmt:'{}'", stream, fmt);
            call.set_return(emu, fmt.len() as u64);
        }
        "sscanf" | "__stdio_common_vsscanf" => {
            let buf_ptr = api_try!(emu, call.arg(emu, 0));
            let fmt_ptr = api_try!(emu, call.arg(emu, 1));
            let buf = emu.maps.read_string(buf_ptr);
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "wincrt!{} buf:'{}' fmt:'{}'", api, buf, fmt);
            call.set_return(emu, 1);
        }
        "__stdio_common_vsprintf" | "__stdio_common_vsprintf_s" => {
            let _options = api_try!(emu, call.arg(emu, 0));
            let buf = api_try!(emu, call.arg(emu, 1));
            let _buf_count = api_try!(emu, call.arg(emu, 2));
            let fmt_ptr = api_try!(emu, call.arg(emu, 3));
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "wincrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            call.set_return(emu, fmt.len() as u64);
        }
        _ => {
            log_red!(emu, "wincrt!{}", api);
            call.set_return(emu, 0);
        }
    }
}
