use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ILCreateFromPath(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "shell32!ILCreateFromPath arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("ilcreatefrompath");
    call.set_return(emu, handle);
    call.cleanup_args(emu, 2);
}

pub fn ILCreateFromPathA(emu: &mut emu::Emu) {
    il_create_from_path(emu, "ILCreateFromPathA", false);
}

pub fn ILCreateFromPathW(emu: &mut emu::Emu) {
    il_create_from_path(emu, "ILCreateFromPathW", true);
}

fn il_create_from_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("ilcreatefrompathw");
        call.set_return(emu, handle);
        call.cleanup_args(emu, 1);
    } else {
        let handle = helper::handler_create("ilcreatefrompatha");
        call.set_return(emu, handle);
        call.cleanup_args(emu, 1);
    }
}
