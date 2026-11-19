use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SleepConditionVariableSRW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cv = api_try!(emu, call.arg(emu, 0));
    let lock = api_try!(emu, call.arg(emu, 1));
    let ms = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "kernel32!SleepConditionVariableSRW cv:0x{:x} lock:0x{:x} ms:{} flags:{}",
        cv,
        lock,
        ms,
        flags
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
