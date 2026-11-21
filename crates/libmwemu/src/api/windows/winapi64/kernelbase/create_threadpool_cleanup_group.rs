use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpoolCleanupGroup(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernelbase!CreateThreadpoolCleanupGroup");
    let hndl = helper::handler_create("threadpool_cleanup_group");
    call.set_return(emu, hndl);
}
