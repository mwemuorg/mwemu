use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{io_length, read_into, set_errno};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn fread(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let destination = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let file = api_try!(emu, call.arg(emu, 3));
    let Some(total) = io_length(emu, size, count) else {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, 0);
        return;
    };
    if total == 0 {
        call.set_return(emu, 0);
        return;
    }
    if !emu
        .maps
        .can_access_range(destination, total, Permission::WRITE)
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let read = read_into(emu, runtime, file, destination, total);
    log_red!(
        emu,
        "{}!fread destination={destination:#x} size={size} count={count} file={file:#x} => {read:?}",
        runtime.library()
    );
    match read {
        Some(bytes) => call.set_return(emu, bytes as u64 / size),
        None => {
            set_errno(emu, runtime, EBADF);
            call.set_return(emu, 0);
        }
    }
}
