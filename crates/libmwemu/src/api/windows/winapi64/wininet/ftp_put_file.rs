use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FtpPutFileA(emu: &mut emu::Emu) {
    ftp_put_file(emu, "FtpPutFileA", false);
}

fn ftp_put_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ftp_handle = api_try!(emu, call.arg(emu, 0));
    let local_ptr = api_try!(emu, call.arg(emu, 1));
    let remote_ptr = api_try!(emu, call.arg(emu, 2));
    let local = if local_ptr != 0 {
        strings::read_text(emu, local_ptr, wide)
    } else {
        String::new()
    };
    let remote = if remote_ptr != 0 {
        strings::read_text(emu, remote_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} local:'{}' remote:'{}'",
        ftp_handle,
        local,
        remote
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn FtpPutFileW(emu: &mut emu::Emu) {
    ftp_put_file(emu, "FtpPutFileW", true);
}
