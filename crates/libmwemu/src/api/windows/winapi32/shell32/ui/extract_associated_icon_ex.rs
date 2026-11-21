use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ExtractAssociatedIconExA(emu: &mut emu::Emu) {
    extract_associated_icon_ex(emu, "ExtractAssociatedIconExA", false);
}

pub fn ExtractAssociatedIconExW(emu: &mut emu::Emu) {
    extract_associated_icon_ex(emu, "ExtractAssociatedIconExW", true);
}

fn extract_associated_icon_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("extractassociatediconexw");
        call.set_return(emu, handle);
        call.cleanup_args(emu, 4);
    } else {
        let handle = helper::handler_create("extractassociatediconexa");
        call.set_return(emu, handle);
        call.cleanup_args(emu, 4);
    }
}
