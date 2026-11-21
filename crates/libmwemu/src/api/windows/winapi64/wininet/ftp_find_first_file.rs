use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FtpFindFirstFileA(emu: &mut emu::Emu) {
    ftp_find_first_file(emu, "FtpFindFirstFileA", false);
}

fn ftp_find_first_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ftp_handle = api_try!(emu, call.arg(emu, 0));
    let search_ptr = api_try!(emu, call.arg(emu, 1));
    let find_data_ptr = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let search = if search_ptr != 0 {
        strings::read_text(emu, search_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} search:'{}'",
        ftp_handle,
        search
    );
    emu.set_last_error(18);
    call.set_return(emu, 0xFFFFFFFFFFFFFFFF);
}

pub fn FtpFindFirstFileW(emu: &mut emu::Emu) {
    ftp_find_first_file(emu, "FtpFindFirstFileW", true);
}
