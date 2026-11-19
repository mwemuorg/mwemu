use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WinHttpCloseHandle(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_internet = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "winhttp!WinHttpCloseHandle handle: 0x{:x}", h_internet);

    helper::handler_close(h_internet);
    call.set_return(emu, 1);
}
