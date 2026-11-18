use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::windows::constants::errors::*;

pub fn _guard_check_icall_nop(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!_guard_check_icall_nop");

    call.set_return(emu, ERROR_SUCCESS);
}
