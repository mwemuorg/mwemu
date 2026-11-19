use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetThreadAffinityMask(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let mask = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!SetThreadAffinityMask h:0x{:x} mask:0x{:x}",
        handle,
        mask
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // previous mask
}
