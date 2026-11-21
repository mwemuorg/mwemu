use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn TlsAlloc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    emu.tls32_mut().push(0);
    let idx = (emu.tls32().len() - 1) as u32;
    call.set_return(emu, idx as u64);

    log_red!(emu, "kernel32!TlsAlloc = {}", idx);
}
