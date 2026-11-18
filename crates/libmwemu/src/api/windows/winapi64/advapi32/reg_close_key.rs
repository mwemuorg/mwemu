use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegCloseKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    helper::handler_close(hkey);
    log_red!(emu, "advapi32!RegCloseKey 0x{:x}", hkey);
    call.set_return(emu, 0);
}
