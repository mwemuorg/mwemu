use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetGetLastResponseInfoA(emu: &mut emu::Emu) {
    internet_get_last_response_info(emu, "InternetGetLastResponseInfoA");
}

fn internet_get_last_response_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let error_ptr = api_try!(emu, call.arg(emu, 0));
    let buf_ptr = api_try!(emu, call.arg(emu, 1));
    let buf_len_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "wininet!{api}");
    if error_ptr != 0 {
        emu.maps.write_dword(error_ptr, 0);
    }
    if buf_len_ptr != 0 {
        emu.maps.write_dword(buf_len_ptr, 0);
    }
    call.set_return(emu, 1);
}

pub fn InternetGetLastResponseInfoW(emu: &mut emu::Emu) {
    internet_get_last_response_info(emu, "InternetGetLastResponseInfoW");
}
