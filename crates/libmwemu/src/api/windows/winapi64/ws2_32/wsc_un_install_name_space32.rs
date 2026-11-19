use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSCUnInstallNameSpace32(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!WSCUnInstallNameSpace32 ptr:0x{:x}", err_ptr);
    let handle = helper::handler_create("WSCUnInstallNameSpace32");
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
