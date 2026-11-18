use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn K32GetProcessImageFileNameA(emu: &mut emu::Emu) {
    k32_get_process_image_file_name(emu, "K32GetProcessImageFileNameA", false);
}

fn k32_get_process_image_file_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernel32!{api} '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 3);
    call.set_return(emu, 1);
}

pub fn K32GetProcessImageFileNameW(emu: &mut emu::Emu) {
    k32_get_process_image_file_name(emu, "K32GetProcessImageFileNameW", true);
}
