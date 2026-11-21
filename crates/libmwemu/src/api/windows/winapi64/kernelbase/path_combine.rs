use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathCombineA(emu: &mut emu::Emu) {
    path_combine(emu, "PathCombineA", false);
}

fn path_combine(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let dst: u64 = api_try!(emu, call.arg(emu, 0));
    let dir = api_try!(emu, call.arg(emu, 1));
    let file = api_try!(emu, call.arg(emu, 2));

    let mut path1 = String::new();
    let mut path2 = String::new();

    if dir > 0 {
        path1 = strings::read_text(emu, dir, wide);
    }
    if file > 0 {
        path2 = strings::read_text(emu, file, wide);
    }

    log_red!(emu, "kernelbase!{api} path1: {} path2: {}", path1, path2);

    if dst != 0 && !path1.is_empty() && !path2.is_empty() {
        strings::write_string(emu, dst, &format!("{}\\{}", path1, path2), wide);
    }

    call.set_return(emu, dst);
}

pub fn PathCombineW(emu: &mut emu::Emu) {
    path_combine(emu, "PathCombineW", true);
}
