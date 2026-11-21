use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesA", false);
}

fn get_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let filename_ptr = api_try!(emu, call.arg(emu, 0));
    let filename = strings::read_text(emu, filename_ptr, wide);

    log_red!(emu, "kernel32!{api} file: '{}'", filename);

    let attrs: u64 = if filename.ends_with('\\') || filename.ends_with('/') {
        0x10 // FILE_ATTRIBUTE_DIRECTORY
    } else {
        0x80 // FILE_ATTRIBUTE_NORMAL
    };

    emu.set_last_error(0);
    call.set_return(emu, attrs);
}

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    get_file_attributes(emu, "GetFileAttributesW", true);
}
