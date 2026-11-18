use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ReleaseSRWLockShared(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lock_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!ReleaseSRWLockShared lock:0x{:x}", lock_ptr);
    if lock_ptr != 0 {
        let val = emu.maps.read_dword(lock_ptr).unwrap_or(0);
        emu.maps.write_dword(lock_ptr, val.saturating_sub(1));
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
