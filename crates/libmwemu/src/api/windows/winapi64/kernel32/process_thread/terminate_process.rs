use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::process::*;

pub fn TerminateProcess(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hndl = api_try!(emu, call.arg(emu, 0));
    let code = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!TerminateProcess hndl: {} code: {}",
        hndl,
        code
    );
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    if hndl == CURRENT_PROCESS_HANDLE {
        emu.stop();
        return;
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
