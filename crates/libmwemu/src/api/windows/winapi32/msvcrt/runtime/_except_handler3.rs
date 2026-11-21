use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _except_handler3(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let exception_record = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!{} exc_record:0x{:x}", api, exception_record);
    emu.set_last_error(0);
    call.set_return(emu, 1); // ExceptionContinueSearch
}
