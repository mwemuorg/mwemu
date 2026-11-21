use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FlsFree(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!FlsFree idx: {}", idx);
    if (idx as usize) < emu.fls().len() {
        emu.fls_mut()[idx as usize] = 0;
        emu.set_last_error(0);
        call.set_return(emu, 1);
    } else {
        emu.set_last_error(0);
        call.set_return(emu, 0);
    }
}
