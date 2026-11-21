use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn RegisterTypeLib(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _a0 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "oleaut32!RegisterTypeLib a0:0x{:x}", _a0);
    let _h = helper::handler_create("oleaut32://RegisterTypeLib");
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}
