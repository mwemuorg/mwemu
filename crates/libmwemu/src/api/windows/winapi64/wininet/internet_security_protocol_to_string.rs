use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetSecurityProtocolToStringA(emu: &mut emu::Emu) {
    internet_security_protocol_to_string(emu, "InternetSecurityProtocolToStringA", false);
}

fn internet_security_protocol_to_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let protocol = api_try!(emu, call.arg_u32(emu, 0));
    let buf_ptr = api_try!(emu, call.arg(emu, 1));
    let buf_len_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "wininet!{api} protocol:{}", protocol);
    let src = api_try!(emu, call.arg(emu, 0));
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = strings::read_text(emu, src, wide);
    }
    if buf_ptr != 0 {
        strings::write_string(emu, buf_ptr, "TLS 1.2", wide);
    }
    call.set_return(emu, 1);
}

pub fn InternetSecurityProtocolToStringW(emu: &mut emu::Emu) {
    internet_security_protocol_to_string(emu, "InternetSecurityProtocolToStringW", true);
}
