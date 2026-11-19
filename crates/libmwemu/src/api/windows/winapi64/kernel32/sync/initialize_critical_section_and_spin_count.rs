use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
pub fn InitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let crit_sect = api_try!(emu, call.arg(emu, 0));
    let spin_count = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!InitializeCriticalSectionAndSpinCount crit_sect: 0x{:x} spin_count: {}",
        crit_sect,
        spin_count
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
