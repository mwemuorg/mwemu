use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadAcceleratorsA(emu: &mut emu::Emu) {
    load_accelerators(emu, "LoadAcceleratorsA");
}

fn load_accelerators(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "user32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let hndl = helper::handler_create(&format!("user32://{api}"));
    call.set_return(emu, hndl);
}

pub fn LoadAcceleratorsW(emu: &mut emu::Emu) {
    load_accelerators(emu, "LoadAcceleratorsW");
}
