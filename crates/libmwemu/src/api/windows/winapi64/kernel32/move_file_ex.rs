use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn MoveFileExA(emu: &mut emu::Emu) {
    move_file_ex(emu, "MoveFileExA", false);
}

fn move_file_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let src = api_try!(emu, call.arg(emu, 0));
    let dst = api_try!(emu, call.arg(emu, 1));
    let src_s = strings::read_text(emu, src, wide);
    let dst_s = strings::read_text(emu, dst, wide);
    log_red!(emu, "kernel32!{api} '{}' -> '{}'", src_s, dst_s);
    call.set_return(emu, 1);
}

pub fn MoveFileExW(emu: &mut emu::Emu) {
    move_file_ex(emu, "MoveFileExW", true);
}
