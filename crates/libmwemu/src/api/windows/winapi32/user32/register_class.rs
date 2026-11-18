use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn RegisterClassA(emu: &mut emu::Emu) {
    register_class(emu, "RegisterClassA");
}

fn register_class(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let wndclass = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "user32!{api} 0x{:x}", wndclass);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}

pub(super) fn RegisterClassW(emu: &mut emu::Emu) {
    register_class(emu, "RegisterClassW");
}
