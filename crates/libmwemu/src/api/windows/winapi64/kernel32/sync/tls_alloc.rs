use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn TlsAlloc(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!TlsAlloc");
    let _h = helper::handler_create("tls_slot");

    emu.tls64_mut().push(0);
    emu.set_last_error(0);
    let return_value = (emu.tls64().len() - 1) as u64;
    call.set_return(emu, return_value); // Return index of newly allocated slot
}
