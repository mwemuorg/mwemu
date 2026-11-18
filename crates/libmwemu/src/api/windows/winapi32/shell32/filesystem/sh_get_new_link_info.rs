use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SHGetNewLinkInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "shell32!SHGetNewLinkInfo arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}

pub fn SHGetNewLinkInfoA(emu: &mut emu::Emu) {
    sh_get_new_link_info(emu, "SHGetNewLinkInfoA");
}

fn sh_get_new_link_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}

pub fn SHGetNewLinkInfoW(emu: &mut emu::Emu) {
    sh_get_new_link_info(emu, "SHGetNewLinkInfoW");
}
