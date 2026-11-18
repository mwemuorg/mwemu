use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetProcessPriorityBoost(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let disable_ptr = api_try!(emu, call.arg(emu, 1));
    if disable_ptr != 0 {
        emu.maps.write_dword(disable_ptr, 0); // boost not disabled
    }
    log_red!(
        emu,
        "kernelbase!GetProcessPriorityBoost h:0x{:x} => FALSE",
        handle
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
