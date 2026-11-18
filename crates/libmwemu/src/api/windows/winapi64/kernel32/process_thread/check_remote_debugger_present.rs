use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::process::*;

pub fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let process = api_try!(emu, call.arg(emu, 0));
    let present_ptr = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "kernel32!CheckRemoteDebuggerPresent hProcess: 0x{:x} pbDebuggerPresent: 0x{:x}",
        process,
        present_ptr
    );

    if process == 0 || present_ptr == 0 {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        call.set_return(emu, 0);
        return;
    }

    let valid =
        process == CURRENT_PROCESS_HANDLE || process == u64::MAX || helper::handler_exist(process);
    if !valid {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        call.set_return(emu, 0);
        return;
    }

    emu.maps.write_dword(present_ptr, 0);
    call.set_return(emu, 1);
}
