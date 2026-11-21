use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn WSAGetLastError(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err = emu.get_last_error();
    log_red!(emu, "ws2_32!WSAGetLastError => {}", err);
    call.set_return(emu, err);
}
