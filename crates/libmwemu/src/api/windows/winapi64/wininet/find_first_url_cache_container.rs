use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindFirstUrlCacheContainerA(emu: &mut emu::Emu) {
    find_first_url_cache_container(emu, "FindFirstUrlCacheContainerA", false);
}

pub fn FindFirstUrlCacheContainerW(emu: &mut emu::Emu) {
    find_first_url_cache_container(emu, "FindFirstUrlCacheContainerW", true);
}

fn find_first_url_cache_container(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "wininet!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("findfirsturlcachecontainerw");
        call.set_return(emu, handle);
    } else {
        let handle = helper::handler_create("findfirsturlcachecontainera");
        call.set_return(emu, handle);
    }
}
