use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FtpCommandA(emu: &mut emu::Emu) {
    ftp_command(emu, "FtpCommandA", false);
}

fn ftp_command(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let ftp_handle = api_try!(emu, call.arg(emu, 0));
    let expect_response = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    let cmd_ptr = api_try!(emu, call.arg(emu, 3));
    let cmd = if cmd_ptr != 0 {
        strings::read_text(emu, cmd_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} handle:0x{:x} cmd:'{}'", ftp_handle, cmd);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn FtpCommandW(emu: &mut emu::Emu) {
    ftp_command(emu, "FtpCommandW", true);
}
