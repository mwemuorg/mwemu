use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FindNextUrlCacheEntryA(emu: &mut emu::Emu) {
    find_next_url_cache_entry(emu, "FindNextUrlCacheEntryA");
}

fn find_next_url_cache_entry(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    call.set_return(emu, 0);
}

pub fn FindNextUrlCacheEntryW(emu: &mut emu::Emu) {
    find_next_url_cache_entry(emu, "FindNextUrlCacheEntryW");
}
