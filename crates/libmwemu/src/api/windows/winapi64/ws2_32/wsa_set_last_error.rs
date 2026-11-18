use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSASetLastError(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let error_code = api_try!(emu, call.arg_u32(emu, 0));
    helper::advance_tick(emu, 1);
    emu.set_last_error(error_code as u64);
    log_red!(emu, "ws2_32!WSASetLastError error: {}", error_code);
}
