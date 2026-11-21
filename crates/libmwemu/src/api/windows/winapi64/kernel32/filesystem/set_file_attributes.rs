use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SetFileAttributesA(emu: &mut emu::Emu) {
    set_file_attributes(emu, "SetFileAttributesA", false);
}

fn set_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path = strings::read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let attrs = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "kernel32!{api} '{}' attrs:0x{:x}", path, attrs);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn SetFileAttributesW(emu: &mut emu::Emu) {
    set_file_attributes(emu, "SetFileAttributesW", true);
}
