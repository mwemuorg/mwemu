use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn MoveFileA(emu: &mut emu::Emu) {
    move_file(emu, "MoveFileA", false);
}

fn move_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let src_ptr = api_try!(emu, call.arg(emu, 0));
    let dst_ptr = api_try!(emu, call.arg(emu, 1));

    let src = strings::read_text(emu, src_ptr, wide);
    let dst = strings::read_text(emu, dst_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}` to `{}`", src, dst);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn MoveFileW(emu: &mut emu::Emu) {
    move_file(emu, "MoveFileW", true);
}
