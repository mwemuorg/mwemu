use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SysAddRefString(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _a0 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "oleaut32!SysAddRefString a0:0x{:x}", _a0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
