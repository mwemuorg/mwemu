use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DeleteUrlCacheEntry(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "wininet!DeleteUrlCacheEntry handle:0x{:x}", handle);
    helper::handler_close(handle);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}

pub fn DeleteUrlCacheEntryA(emu: &mut emu::Emu) {
    delete_url_cache_entry(emu, "DeleteUrlCacheEntryA");
}

fn delete_url_cache_entry(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "wininet!{api} handle:0x{:x}", handle);
    helper::handler_close(handle);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}

pub fn DeleteUrlCacheEntryW(emu: &mut emu::Emu) {
    delete_url_cache_entry(emu, "DeleteUrlCacheEntryW");
}
