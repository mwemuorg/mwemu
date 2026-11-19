use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::windows::constants::errors::*;

pub fn UnregisterApplicationRecoveryCallback(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!UnregisterApplicationRecoveryCallback");

    call.set_return(emu, ERROR_SUCCESS);
}
