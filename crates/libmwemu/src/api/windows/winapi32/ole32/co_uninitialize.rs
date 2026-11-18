use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn CoUninitialize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "ole32!CoUninitialize");
    call.set_return(emu, 0);
}
