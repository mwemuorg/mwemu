use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesA", false);
}

fn get_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = strings::read_text(emu, path_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}'", path);
    emu.set_last_error(0);
    call.set_return(emu, 0x80);
}

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesW", true);
}
