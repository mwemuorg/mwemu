use super::errno_cell;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub(super) fn errno_location(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let return_value = errno_cell(emu);
    call.set_return(emu, return_value);
}
