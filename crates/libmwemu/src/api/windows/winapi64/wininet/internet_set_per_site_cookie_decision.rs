use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetSetPerSiteCookieDecisionA(emu: &mut emu::Emu) {
    internet_set_per_site_cookie_decision(emu, "InternetSetPerSiteCookieDecisionA", false);
}

fn internet_set_per_site_cookie_decision(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "wininet!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = strings::read_text(emu, val_ptr, wide);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn InternetSetPerSiteCookieDecisionW(emu: &mut emu::Emu) {
    internet_set_per_site_cookie_decision(emu, "InternetSetPerSiteCookieDecisionW", true);
}
