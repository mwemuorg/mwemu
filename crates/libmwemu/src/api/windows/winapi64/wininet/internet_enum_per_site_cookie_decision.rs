use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetEnumPerSiteCookieDecisionA(emu: &mut emu::Emu) {
    internet_enum_per_site_cookie_decision(emu, "InternetEnumPerSiteCookieDecisionA");
}

fn internet_enum_per_site_cookie_decision(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let buf_ptr = api_try!(emu, call.arg(emu, 0));
    let buf_size = api_try!(emu, call.arg(emu, 1));
    let index_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "wininet!{api} buf:0x{:x} idx:0x{:x}",
        buf_ptr,
        index_ptr
    );
    let out_ptr = api_try!(emu, call.arg(emu, 2));
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(259);
    call.set_return(emu, 0);
}

pub fn InternetEnumPerSiteCookieDecisionW(emu: &mut emu::Emu) {
    internet_enum_per_site_cookie_decision(emu, "InternetEnumPerSiteCookieDecisionW");
}
