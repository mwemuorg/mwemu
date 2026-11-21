use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::with_stream;
use crate::winapi::policy::api_try;

pub(super) fn clearerr(emu: &mut emu::Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{}!clearerr 0x{:x}", runtime.library(), file);
    with_stream(emu, runtime, file, |s| {
        s.eof = false;
        s.error = false;
    });
}
