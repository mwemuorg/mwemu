use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadResource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("resource_data");
    log_red!(emu, "kernelbase!LoadResource p1:0x{:x} => 0x{:x}", p1, hndl);
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
