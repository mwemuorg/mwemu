use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WahOpenApcHelper(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let helper_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!WahOpenApcHelper ptr:0x{:x}", helper_ptr);
    let handle = helper::handler_create("wah_handle");
    if helper_ptr != 0 {
        emu.maps.write_qword(helper_ptr, handle);
    }
    call.set_return(emu, 0);
}
