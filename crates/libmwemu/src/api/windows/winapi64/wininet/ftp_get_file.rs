use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FtpGetFileA(emu: &mut emu::Emu) {
    ftp_get_file(emu, "FtpGetFileA", false);
}

fn ftp_get_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ftp_handle = api_try!(emu, call.arg(emu, 0));
    let remote_ptr = api_try!(emu, call.arg(emu, 1));
    let local_ptr = api_try!(emu, call.arg(emu, 2));
    let remote = if remote_ptr != 0 {
        strings::read_text(emu, remote_ptr, wide)
    } else {
        String::new()
    };
    let local = if local_ptr != 0 {
        strings::read_text(emu, local_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} remote:'{}' local:'{}'",
        ftp_handle,
        remote,
        local
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn FtpGetFileW(emu: &mut emu::Emu) {
    ftp_get_file(emu, "FtpGetFileW", true);
}
