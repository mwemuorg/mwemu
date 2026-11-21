use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn DispatchMessageA(emu: &mut emu::Emu) {
    dispatch_message(emu, "DispatchMessageA", false);
}

pub(super) fn DispatchMessageW(emu: &mut emu::Emu) {
    dispatch_message(emu, "DispatchMessageW", true);
}

fn dispatch_message(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!{api} arg1:0x{:x}", arg1);
    if wide {
    } else {
        emu.set_last_error(0);
    }

    call.set_return(emu, 0);
}
