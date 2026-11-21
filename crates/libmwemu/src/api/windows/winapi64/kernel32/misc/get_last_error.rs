use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetLastError(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err = emu.get_last_error();
    log_red!(emu, "kernel32!GetLastError -> {}", err);
    call.set_return(emu, err);
}
