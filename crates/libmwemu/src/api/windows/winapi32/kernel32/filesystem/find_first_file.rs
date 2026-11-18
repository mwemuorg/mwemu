use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    find_first_file(emu, "FindFirstFileA", false);
}

pub fn FindFirstFileW(emu: &mut emu::Emu) {
    find_first_file(emu, "FindFirstFileW", true);
}

fn find_first_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let file_ptr = api_try!(emu, call.arg(emu, 0));
    let find_data = api_try!(emu, call.arg_u32(emu, 1));
    let file = strings::read_text(emu, file_ptr, wide);
    log_red!(emu, "kernel32!{api} file: {}", file);
    call.cleanup_args(emu, 2);
    if wide {
        call.set_return(emu, helper::handler_create(&file));
    } else {
        call.set_return(emu, 1);
    }
}
