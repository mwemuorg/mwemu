use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitializeSRWLock(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lock_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!InitializeSRWLock ptr: 0x{:x}", lock_ptr);

    emu.maps.write_qword(lock_ptr, 0);
}
