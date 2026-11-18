use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::process::*;

pub fn TerminateProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg_u32(emu, 0));
    let code = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(
        emu,
        "kernel32!TerminateProcess hndl: {} code: {}",
        hndl,
        code
    );

    call.cleanup_args(emu, 2);

    if hndl == CURRENT_PROCESS_HANDLE as u32 {
        emu.stop();
        return;
    }
    call.set_return(emu, 1);
}
