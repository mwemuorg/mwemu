use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;

pub fn GetActiveProcessorGroupCount(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "processor group count: 1");
    call.set_return(emu, 1);
}
pub use GetActiveProcessorGroupCount as GetMaximumProcessorGroupCount;
