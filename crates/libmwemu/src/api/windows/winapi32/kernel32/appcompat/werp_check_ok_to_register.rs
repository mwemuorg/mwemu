use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::windows::constants::errors::*;

pub fn WerpCheckOkToRegister(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!WerpCheckOkToRegister");

    call.set_return(emu, ERROR_SUCCESS);
}
