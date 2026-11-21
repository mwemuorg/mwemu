use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetFileAttributesA(emu: &mut emu::Emu) {
    set_file_attributes(emu, "SetFileAttributesA", false);
}

fn set_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let attrs = api_try!(emu, call.arg_u32(emu, 1));
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} '{}' attrs:0x{:x}", path, attrs);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}

pub fn SetFileAttributesW(emu: &mut emu::Emu) {
    set_file_attributes(emu, "SetFileAttributesW", true);
}
