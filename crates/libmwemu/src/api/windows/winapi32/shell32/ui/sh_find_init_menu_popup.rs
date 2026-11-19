use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SHFind_InitMenuPopup(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "shell32!SHFind_InitMenuPopup arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shfind_initmenupopup");
    call.set_return(emu, handle);
    call.cleanup_args(emu, 4);
}
