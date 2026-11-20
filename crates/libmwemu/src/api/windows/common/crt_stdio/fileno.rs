use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{set_errno, with_stream};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn fileno(emu: &mut emu::Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    let fd = with_stream(emu, runtime, file, |s| s.fd);
    log_red!(emu, "{}!_fileno 0x{:x} = {:?}", runtime.library(), file, fd);
    match fd {
        Some(fd) if fd >= 0 => call.set_return(emu, fd as u64),
        _ => {
            set_errno(emu, runtime, EINVAL);
            call.set_return(emu, EOF);
        }
    }
}
