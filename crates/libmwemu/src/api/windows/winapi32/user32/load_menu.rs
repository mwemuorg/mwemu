use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadMenuA(emu: &mut emu::Emu) {
    load_menu(emu, "LoadMenuA", false);
}

pub fn LoadMenuW(emu: &mut emu::Emu) {
    load_menu(emu, "LoadMenuW", true);
}

fn load_menu(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    if wide {
        call.set_return(emu, helper::handler_create("loadmenuw"));
    } else {
        call.set_return(emu, helper::handler_create("loadmenua"));
    }

    call.cleanup_args(emu, 2);
}
