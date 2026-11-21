use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CopyFileA(emu: &mut emu::Emu) {
    copy_file(emu, "CopyFileA", false);
}

fn copy_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let src_ptr = api_try!(emu, call.arg(emu, 0));
    let dst_ptr = api_try!(emu, call.arg(emu, 1));
    let do_fail = api_try!(emu, call.arg(emu, 2));

    let src = strings::read_text(emu, src_ptr, wide);
    let dst = strings::read_text(emu, dst_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}` to `{}`", src, dst);

    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn CopyFileW(emu: &mut emu::Emu) {
    copy_file(emu, "CopyFileW", true);
}
