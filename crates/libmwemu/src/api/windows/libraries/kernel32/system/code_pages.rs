use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::windows::constants::nls::{CP_IBM437, CP_WINDOWS_1252};

pub fn GetACP(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetACP");
    call.set_return(emu, CP_WINDOWS_1252 as u64);
}

pub fn GetOEMCP(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetOEMCP");
    call.set_return(emu, CP_IBM437 as u64);
}
