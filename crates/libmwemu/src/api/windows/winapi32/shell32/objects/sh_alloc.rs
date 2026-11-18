use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SHAlloc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "shell32!SHAlloc arg1:0x{:x}", arg1);
    let handle = helper::handler_create("SHAlloc");
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 1);
}
