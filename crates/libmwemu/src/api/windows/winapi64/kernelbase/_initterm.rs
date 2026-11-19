use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn _initterm(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernelbase!_initterm arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    call.set_return(emu, 0);
}
