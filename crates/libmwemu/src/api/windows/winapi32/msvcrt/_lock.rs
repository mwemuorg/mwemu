use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _lock(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let lock_num = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "msvcrt!_lock {}", lock_num);

    emu.set_last_error(0);
    call.set_return(emu, 1);
}
