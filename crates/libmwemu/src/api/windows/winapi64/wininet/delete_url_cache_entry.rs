use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DeleteUrlCacheEntry(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    helper::handler_close(handle);
    log_red!(emu, "wininet!DeleteUrlCacheEntry handle:0x{:x}", handle);
    call.set_return(emu, 1);
}

pub fn DeleteUrlCacheEntryA(emu: &mut emu::Emu) {
    delete_url_cache_entry(emu, "DeleteUrlCacheEntryA");
}

fn delete_url_cache_entry(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wininet!{api} handle:0x{:x}", handle);
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn DeleteUrlCacheEntryW(emu: &mut emu::Emu) {
    delete_url_cache_entry(emu, "DeleteUrlCacheEntryW");
}
