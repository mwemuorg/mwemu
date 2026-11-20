use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{set_errno, with_stream};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn fflush(emu: &mut emu::Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    let ok = file == 0 || with_stream(emu, runtime, file, |s| s.fd >= 0) == Some(true);
    if !ok {
        set_errno(emu, runtime, EBADF);
    }
    log_red!(emu, "{}!fflush 0x{:x}", runtime.library(), file);
    call.set_return(emu, if ok { 0 } else { EOF });
}
