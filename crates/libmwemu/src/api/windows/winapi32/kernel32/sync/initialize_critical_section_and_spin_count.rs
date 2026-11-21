use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let crit_sect = api_try!(emu, call.arg_u32(emu, 0));
    let spin_count = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionAndSpinCount crit_sect: 0x{:x} spin_count: {}",
        crit_sect,
        spin_count
    );

    call.cleanup_args(emu, 2);

    call.set_return(emu, 1);
}
