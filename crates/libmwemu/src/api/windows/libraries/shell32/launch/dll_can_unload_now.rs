use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::windows::constants::errors::S_FALSE;

pub fn DllCanUnloadNow(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "shell32!DllCanUnloadNow");
    call.set_return(emu, S_FALSE);
}
