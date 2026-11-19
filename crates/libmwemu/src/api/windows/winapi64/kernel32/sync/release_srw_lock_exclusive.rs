use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ReleaseSRWLockExclusive(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lock_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "kernel32!ReleaseSRWLockExclusive lock:0x{:x}",
        lock_ptr
    );
    if lock_ptr != 0 {
        emu.maps.write_dword(lock_ptr, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
