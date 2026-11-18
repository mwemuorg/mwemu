use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetFileAttributesExA(emu: &mut emu::Emu) {
    get_file_attributes_ex(emu, "GetFileAttributesExA", false);
}

fn get_file_attributes_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let info_level = api_try!(emu, call.arg(emu, 1));
    let file_info_ptr = api_try!(emu, call.arg(emu, 2));
    let path = strings::read_text(emu, path_ptr, wide);
    if file_info_ptr != 0 {
        emu.maps.write_dword(file_info_ptr, 0x80); // dwFileAttributes = NORMAL
        emu.maps.write_dword(file_info_ptr + 28, 0); // nFileSizeHigh
        emu.maps.write_dword(file_info_ptr + 32, 4096); // nFileSizeLow
    }
    log_red!(emu, "kernelbase!{api} '{}'", path);
    call.set_return(emu, 1);
}

pub fn GetFileAttributesExW(emu: &mut emu::Emu) {
    get_file_attributes_ex(emu, "GetFileAttributesExW", true);
}
