use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FreeAddrInfoEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    helper::handler_close(handle);
    log_red!(emu, "ws2_32!FreeAddrInfoEx handle:0x{:x}", handle);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}

pub fn FreeAddrInfoExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let info_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!FreeAddrInfoExW ptr:0x{:x}", info_ptr);
    helper::handler_close(info_ptr);
    call.cleanup_args(emu, 1);
}
