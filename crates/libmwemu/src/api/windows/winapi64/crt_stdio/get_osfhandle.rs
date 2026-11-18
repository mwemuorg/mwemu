use super::EBADF;
use super::INVALID_HANDLE_VALUE;
use super::fd_handle;
use super::set_errno;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn get_osfhandle(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    let handle = fd_handle(fd);
    log_red!(emu, "wincrt!_get_osfhandle fd {} = {:?}", fd, handle);
    match handle {
        Some(handle) => call.set_return(emu, handle as u64),
        None => {
            set_errno(emu, EBADF);
            call.set_return(emu, INVALID_HANDLE_VALUE);
        }
    }
}
