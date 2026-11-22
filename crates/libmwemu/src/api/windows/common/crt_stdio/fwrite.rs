use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{descriptors, io_length, set_errno, with_stream};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn fwrite(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let source = api_try!(emu, call.arg(emu, 0));
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
    if !emu.maps.can_access_range(source, total, Permission::READ) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let Some(fd) = with_stream(emu, runtime, file, |stream| stream.fd) else {
        set_errno(emu, runtime, EBADF);
        call.set_return(emu, 0);
        return;
    };
    let (written, error) = match descriptors::write_bytes(emu, runtime, fd, source, total) {
        Ok(count) => (count, (count < total).then_some(EIO)),
        Err(error) => (0, Some(error)),
    };
    if let Some(error) = error {
        with_stream(emu, runtime, file, |stream| stream.error = true);
        set_errno(emu, runtime, error);
    }
    log_red!(
        emu,
        "{}!fwrite source={source:#x} size={size} count={count} file={file:#x} => {written}",
        runtime.library()
    );
    call.set_return(emu, written as u64 / size);
}
