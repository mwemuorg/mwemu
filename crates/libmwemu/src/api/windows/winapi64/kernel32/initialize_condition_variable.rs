use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitializeConditionVariable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cv = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!InitializeConditionVariable 0x{:x}", cv);
    if cv != 0 {
        emu.maps.write_qword(cv, 0);
    }
}
