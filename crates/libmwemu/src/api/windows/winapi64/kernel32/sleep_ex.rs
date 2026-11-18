use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SleepEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ms = api_try!(emu, call.arg(emu, 0));
    let alertable = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "kernel32!SleepEx ms:{} alertable:{}", ms, alertable);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
