use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
pub fn TryAcquireSRWLockExclusive(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lock_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "kernel32!TryAcquireSRWLockExclusive lock:0x{:x}",
        lock_ptr
    );
    let handle = helper::handler_create("TryAcquireSrwLockExclusive");
    if lock_ptr != 0 {
        let val = emu.maps.read_dword(lock_ptr).unwrap_or(0);
        if val == 0 {
            call.set_return(emu, 1);
        } else {
            call.set_return(emu, 0);
        }
    } else {
        call.set_return(emu, 0);
    }
    emu.set_last_error(0);
}
