use super::EBADF;
use super::EINVAL;
use super::EOF;
use super::fd_handle;
use super::set_errno;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn filelength(emu: &mut emu::Emu, api: &str, long: bool) {
    let call = WinApi(Stdcall);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    let len = fd_handle(fd).and_then(|handle| {
        emu.handle_management
            .get_mut_file_handle(handle)
            .and_then(|fh| fh.length().ok())
    });
    log_red!(emu, "wincrt!{} fd {} = {:?}", api, fd, len);
    match len {
        Some(len) if long && len > i32::MAX as u64 => {
            set_errno(emu, EINVAL);
            call.set_return(emu, EOF);
        }
        Some(len) => call.set_return(emu, len),
        None => {
            set_errno(emu, EBADF);
            call.set_return(emu, if long { EOF } else { u64::MAX });
        }
    }
}
