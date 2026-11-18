use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn UnlockUrlCacheEntryFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "wininet!UnlockUrlCacheEntryFile arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn UnlockUrlCacheEntryFileA(emu: &mut emu::Emu) {
    unlock_url_cache_entry_file(emu, "UnlockUrlCacheEntryFileA");
}

fn unlock_url_cache_entry_file(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "wininet!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn UnlockUrlCacheEntryFileW(emu: &mut emu::Emu) {
    unlock_url_cache_entry_file(emu, "UnlockUrlCacheEntryFileW");
}
