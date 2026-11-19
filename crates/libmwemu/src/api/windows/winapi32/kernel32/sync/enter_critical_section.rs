use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn EnterCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let crit_sect = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "kernel32!EnterCriticalSection 0x{:x}", crit_sect);
    call.set_return(emu, crit_sect as u64);
    call.cleanup_args(emu, 1);
}
