use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FtpSetCurrentDirectoryA(emu: &mut emu::Emu) {
    ftp_set_current_directory(emu, "FtpSetCurrentDirectoryA", false);
}

fn ftp_set_current_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ftp_handle = api_try!(emu, call.arg(emu, 0));
    let dir_ptr = api_try!(emu, call.arg(emu, 1));
    let dir = if dir_ptr != 0 {
        strings::read_text(emu, dir_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} handle:0x{:x} dir:'{}'", ftp_handle, dir);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn FtpSetCurrentDirectoryW(emu: &mut emu::Emu) {
    ftp_set_current_directory(emu, "FtpSetCurrentDirectoryW", true);
}
