use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _CxxThrowException(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let exc_object = api_try!(emu, call.arg(emu, 0));
    let throw_info = api_try!(emu, call.arg(emu, 1));
    let vtable = if exc_object != 0 {
        emu.maps.read_qword(exc_object).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "msvcrt!_CxxThrowException obj:0x{:x} vtable:0x{:x} info:0x{:x}",
        exc_object,
        vtable,
        throw_info
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
