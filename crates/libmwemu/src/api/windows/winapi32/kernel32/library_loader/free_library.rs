use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FreeLibrary(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hmod = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!FreeLibrary   {:x}", hmod);

    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
