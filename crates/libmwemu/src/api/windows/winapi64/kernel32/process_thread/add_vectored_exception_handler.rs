use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
pub fn AddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let first = api_try!(emu, call.arg_u32(emu, 0));
    let handler = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!AddVectoredExceptionHandler first:{} handler:0x{:x}",
        first,
        handler
    );
    emu.add_veh(first != 0, handler);

    // the handler address doubles as the handle RemoveVectoredExceptionHandler gets
    call.set_return(emu, handler);
}
