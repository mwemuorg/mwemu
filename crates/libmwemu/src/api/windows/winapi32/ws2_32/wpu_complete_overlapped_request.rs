use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WPUCompleteOverlappedRequest(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let overlapped_ptr = api_try!(emu, call.arg(emu, 1));
    let error = api_try!(emu, call.arg_u32(emu, 2));
    let bytes = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "ws2_32!WPUCompleteOverlappedRequest sock:0x{:x} err:{} bytes:{}",
        sock,
        error,
        bytes
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 5);
}
