use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FtpOpenFileA(emu: &mut emu::Emu) {
    ftp_open_file(emu, "FtpOpenFileA", false);
}

fn ftp_open_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ftp_handle = api_try!(emu, call.arg(emu, 0));
    let filename_ptr = api_try!(emu, call.arg(emu, 1));
    let access = api_try!(emu, call.arg_u32(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let filename = if filename_ptr != 0 {
        strings::read_text(emu, filename_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} file:'{}' access:0x{:x}",
        ftp_handle,
        filename,
        access
    );
    let uri = format!("ftp://{}", filename);
    call.set_return(emu, helper::handler_create(&uri));
}

pub fn FtpOpenFileW(emu: &mut emu::Emu) {
    ftp_open_file(emu, "FtpOpenFileW", true);
}
