use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateActCtxW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let name = emu.maps.read_wide_string(arg1);
    let hndl = helper::handler_create(&format!("kb_{}", name));
    log_red!(emu, "kernelbase!CreateActCtxW '{}' => 0x{:x}", name, hndl);
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
