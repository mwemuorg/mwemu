use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{set_errno, stream_handle, with_stream};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn fgets(emu: &mut Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let destination = api_try!(emu, call.arg(emu, 0));
    let capacity = api_try!(emu, call.arg_i32(emu, 1));
    let file = api_try!(emu, call.arg(emu, 2));
    let Some(handle) =
        stream_handle(emu, runtime, file).filter(|_| destination != 0 && capacity > 0)
    else {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, 0);
        return;
    };
    if !emu
        .maps
        .can_access_range(destination, capacity as usize, Permission::WRITE)
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let mut written = 0;
    let mut failed = false;
    while written + 1 < capacity as usize {
        let mut byte = [0];
        let result = emu
            .handle_management
            .get_mut_file_handle(handle)
            .map(|file| file.read(&mut byte));
        match result {
            Some(Ok(1)) => {
                if !emu.maps.write_byte(destination + written as u64, byte[0]) {
                    failed = true;
                    break;
                }
                written += 1;
                if byte[0] == b'\n' {
                    break;
                }
            }
            Some(Ok(_)) => {
                with_stream(emu, runtime, file, |stream| stream.eof = true);
                break;
            }
            _ => {
                failed = true;
                break;
            }
        }
    }
    if failed {
        with_stream(emu, runtime, file, |stream| stream.error = true);
        set_errno(emu, runtime, EIO);
    }
    let returns_line = !failed && (written != 0 || capacity == 1);
    if returns_line {
        emu.maps.write_byte(destination + written as u64, 0);
    }
    log_red!(
        emu,
        "{}!fgets destination={destination:#x} capacity={capacity} file={file:#x} => {written}",
        runtime.library()
    );
    call.set_return(emu, if returns_line { destination } else { 0 });
}
