use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::windows::constants::errors::*;

pub fn IsTerminalServerCompatible(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!IsTerminalServerCompatible");

    call.set_return(emu, ERROR_SUCCESS);
}
