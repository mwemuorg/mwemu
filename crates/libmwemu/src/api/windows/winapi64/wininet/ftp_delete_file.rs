use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FtpDeleteFileA(emu: &mut emu::Emu) {
    ftp_delete_file(emu, "FtpDeleteFileA", false);
}

fn ftp_delete_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ftp_handle = api_try!(emu, call.arg(emu, 0));
    let file_ptr = api_try!(emu, call.arg(emu, 1));
    let file = if file_ptr != 0 {
        strings::read_text(emu, file_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} file:'{}'",
        ftp_handle,
        file
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn FtpDeleteFileW(emu: &mut emu::Emu) {
    ftp_delete_file(emu, "FtpDeleteFileW", true);
}
