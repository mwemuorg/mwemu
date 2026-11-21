use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{fd_handle, set_errno};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn filelength(emu: &mut emu::Emu, runtime: Runtime, api: &str, long: bool) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    let len = fd_handle(emu, runtime, fd).and_then(|handle| {
        emu.handle_management
            .get_mut_file_handle(handle)
            .and_then(|fh| fh.length().ok())
    });
    log_red!(emu, "{}!{} fd {} = {:?}", runtime.library(), api, fd, len);
    match len {
        Some(len) if long && len > i32::MAX as u64 => {
            set_errno(emu, runtime, EINVAL);
            call.set_return(emu, EOF);
        }
        Some(len) if long => call.set_return(emu, len),
        Some(len) => call.set_return_u64(emu, len),
        None => {
            set_errno(emu, runtime, EBADF);
            if long {
                call.set_return(emu, EOF);
            } else {
                call.set_return_u64(emu, u64::MAX);
            }
        }
    }
}
