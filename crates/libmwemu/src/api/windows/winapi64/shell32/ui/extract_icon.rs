use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ExtractIconA(emu: &mut emu::Emu) {
    extract_icon(emu, "ExtractIconA", false);
}

pub fn ExtractIconW(emu: &mut emu::Emu) {
    extract_icon(emu, "ExtractIconW", true);
}

fn extract_icon(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("extracticonw");
        emu.set_last_error(0);
        call.set_return(emu, handle);
    } else {
        let handle = helper::handler_create("extracticona");
        emu.set_last_error(0);
        call.set_return(emu, handle);
    }
}
