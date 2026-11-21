use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FtpRenameFileA(emu: &mut emu::Emu) {
    ftp_rename_file(emu, "FtpRenameFileA", false);
}

fn ftp_rename_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ftp_handle = api_try!(emu, call.arg(emu, 0));
    let old_ptr = api_try!(emu, call.arg(emu, 1));
    let new_ptr = api_try!(emu, call.arg(emu, 2));
    let old_name = if old_ptr != 0 {
        strings::read_text(emu, old_ptr, wide)
    } else {
        String::new()
    };
    let new_name = if new_ptr != 0 {
        strings::read_text(emu, new_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} old:'{}' new:'{}'",
        ftp_handle,
        old_name,
        new_name
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn FtpRenameFileW(emu: &mut emu::Emu) {
    ftp_rename_file(emu, "FtpRenameFileW", true);
}
