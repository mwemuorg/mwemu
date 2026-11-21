use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetLastError(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let return_value = emu.get_last_error();
    call.set_return(emu, return_value);
    log_red!(emu, "kernel32!GetLastError ={}", emu.regs().rax);
}
