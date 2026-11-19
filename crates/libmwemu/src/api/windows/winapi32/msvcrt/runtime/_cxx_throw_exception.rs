use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _CxxThrowException(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let exc_object = api_try!(emu, call.arg_u32(emu, 0));
    let throw_info = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "msvcrt!_CxxThrowException obj:0x{:x} info:0x{:x}",
        exc_object,
        throw_info
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}
