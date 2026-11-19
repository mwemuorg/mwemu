use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn LoadStringA(emu: &mut emu::Emu) {
    load_string(emu, "LoadStringA");
}

fn load_string(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let id = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "user32!{api} id:{}", id);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 4);
}

pub(super) fn LoadStringW(emu: &mut emu::Emu) {
    load_string(emu, "LoadStringW");
}
