use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SleepConditionVariableCS(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cv = api_try!(emu, call.arg(emu, 0));
    let cs = api_try!(emu, call.arg(emu, 1));
    let ms = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "kernel32!SleepConditionVariableCS cv:0x{:x} cs:0x{:x} ms:{}",
        cv,
        cs,
        ms
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
