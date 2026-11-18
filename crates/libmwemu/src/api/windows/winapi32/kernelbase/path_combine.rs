use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn PathCombineA(emu: &mut emu::Emu) {
    path_combine(emu, "PathCombineA", false);
}

fn path_combine(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let dir = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let file = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let path1 = if dir != 0 {
        strings::read_text(emu, dir, wide)
    } else {
        String::new()
    };
    let path2 = if file != 0 {
        strings::read_text(emu, file, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!{api} path1: {} path2: {}", path1, path2);
    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        strings::write_string(emu, dst, &format!("{}\\{}", path1, path2), wide);
    }
    call.set_return(emu, dst);
    call.cleanup_args(emu, 3);
}

pub(super) fn PathCombineW(emu: &mut emu::Emu) {
    path_combine(emu, "PathCombineW", true);
}
