use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FtpGetCurrentDirectoryA(emu: &mut emu::Emu) {
    ftp_get_current_directory(emu, "FtpGetCurrentDirectoryA", false);
}

fn ftp_get_current_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let dir_ptr = api_try!(emu, call.arg(emu, 1));
    let dir_len_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "wininet!{api} handle:0x{:x}", handle);
    if dir_ptr != 0 {
        strings::write_string(emu, dir_ptr, "/", wide);
    }
    if dir_len_ptr != 0 {
        emu.maps.write_dword(dir_len_ptr, 1);
    }
    call.set_return(emu, 1);
}

pub fn FtpGetCurrentDirectoryW(emu: &mut emu::Emu) {
    ftp_get_current_directory(emu, "FtpGetCurrentDirectoryW", true);
}
