use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DoSoundDisconnect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "user32!DoSoundDisconnect handle:0x{:x}", handle);
    helper::handler_close(handle);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
