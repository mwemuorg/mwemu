use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VirtualLock(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lp_address = api_try!(emu, call.arg(emu, 0));
    let dw_size = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "kernel32!VirtualLock addr: 0x{:x} size: 0x{:x}",
        lp_address,
        dw_size
    );

    call.cleanup_args(emu, 2);

    // Return TRUE
    call.set_return(emu, 1);
}
