use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn inet_ntop(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "ws2_32!inet_ntop arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    call.cleanup_args(emu, 4);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn InetNtopW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let family = api_try!(emu, call.arg_u32(emu, 0));
    let addr_ptr = api_try!(emu, call.arg(emu, 1));
    let str_buf = api_try!(emu, call.arg(emu, 2));
    let str_len = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "ws2_32!InetNtopW family:{} addr:0x{:x} buf:0x{:x}",
        family,
        addr_ptr,
        str_buf
    );
    if family == 2 && addr_ptr != 0 && str_buf != 0 {
        let ip = emu.maps.read_dword(addr_ptr).unwrap_or(0);
        let a = ip & 0xFF;
        let b = (ip >> 8) & 0xFF;
        let c = (ip >> 16) & 0xFF;
        let d = (ip >> 24) & 0xFF;
        let s = format!("{}.{}.{}.{}", a, b, c, d);
        if (s.len() as u32) < str_len {
            emu.maps.write_wide_string(str_buf, &s);
            call.set_return(emu, str_buf);
        } else {
            call.set_return(emu, 0);
        }
    } else {
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 4);
}
