use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
pub fn TlsFree(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg(emu, 0)) as usize; // First parameter passed in RCX in x64
    log_red!(emu, "kernel32!TlsFree idx: {}", idx);
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    if idx < emu.tls64().len() {
        emu.tls64_mut()[idx] = 0; // Clear the slot
        emu.set_last_error(0);
        call.set_return(emu, 1); // Return TRUE
    } else {
        call.set_return(emu, 0); // Return FALSE if invalid index
    }
}
