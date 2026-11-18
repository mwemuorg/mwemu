use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ConnectNamedPipe(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let overlapped = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!ConnectNamedPipe hndl: 0x{:x}", handle);
    if !helper::handler_exist(handle) {
        log::trace!("\tinvalid handle.");
    }

    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}
