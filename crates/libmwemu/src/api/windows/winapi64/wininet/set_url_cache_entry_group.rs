use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetUrlCacheEntryGroup(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "wininet!SetUrlCacheEntryGroup h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn SetUrlCacheEntryGroupA(emu: &mut emu::Emu) {
    set_url_cache_entry_group(emu, "SetUrlCacheEntryGroupA", false);
}

fn set_url_cache_entry_group(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "wininet!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn SetUrlCacheEntryGroupW(emu: &mut emu::Emu) {
    set_url_cache_entry_group(emu, "SetUrlCacheEntryGroupW", true);
}
