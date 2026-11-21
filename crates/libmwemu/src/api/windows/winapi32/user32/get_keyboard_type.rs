use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetKeyboardType(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ntype = api_try!(emu, call.arg_i32(emu, 0));
    let result: u32 = match ntype {
        0 => 4,
        1 => 0,
        2 => 12,
        _ => 0,
    };
    log_red!(emu, "user32!GetKeyboardType type:{} = {}", ntype, result);
    call.set_return(emu, result as u64);
    call.cleanup_args(emu, 1);
}
