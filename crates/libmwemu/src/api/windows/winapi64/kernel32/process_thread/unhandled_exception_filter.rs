use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::exceptions::*;

pub fn UnhandledExceptionFilter(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let exception_info = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "kernel32!UnhandledExceptionFilter pExceptionInfo:0x{:x}",
        exception_info
    );

    emu.set_last_error(0);
    call.set_return(emu, EXCEPTION_EXECUTE_HANDLER as u64);
    // a debugger would had answered EXCEPTION_CONTINUE_SEARCH
}
