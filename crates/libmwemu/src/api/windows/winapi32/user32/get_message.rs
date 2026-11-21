use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn GetMessageA(emu: &mut emu::Emu) {
    get_message(emu, "GetMessageA");
}

fn get_message(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!{api}");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 4);
}

pub(super) fn GetMessageW(emu: &mut emu::Emu) {
    get_message(emu, "GetMessageW");
}
