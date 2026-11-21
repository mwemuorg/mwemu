use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetUrlCacheGroupAttributeA(emu: &mut emu::Emu) {
    set_url_cache_group_attribute(emu, "SetUrlCacheGroupAttributeA", false);
}

fn set_url_cache_group_attribute(emu: &mut emu::Emu, api: &str, wide: bool) {
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

pub fn SetUrlCacheGroupAttributeW(emu: &mut emu::Emu) {
    set_url_cache_group_attribute(emu, "SetUrlCacheGroupAttributeW", true);
}
