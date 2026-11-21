use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesA", false);
}

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesW", true);
}

fn get_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let filename_ptr = api_try!(emu, call.arg(emu, 0));
    let filename = strings::read_text(emu, filename_ptr, wide);
    log_red!(emu, "kernel32!{api} file: {}", filename);
    if wide {
        call.cleanup_args(emu, 1);
        call.set_return(emu, 0x123);
    } else {
        call.set_return(emu, 0x123);
        call.cleanup_args(emu, 1);
    }
}
