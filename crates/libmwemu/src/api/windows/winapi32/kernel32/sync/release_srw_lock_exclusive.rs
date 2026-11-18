use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ReleaseSRWLockExclusive(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lock_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(
        emu,
        "kernel32!ReleaseSRWLockExclusive ptr: 0x{:x}",
        lock_ptr
    );
    call.cleanup_args(emu, 1);
}
