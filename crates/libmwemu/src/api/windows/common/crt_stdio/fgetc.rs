use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{set_errno, stream_handle, with_stream};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn fgetc(emu: &mut emu::Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    let Some(handle) = stream_handle(emu, runtime, file) else {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, EOF);
        return;
    };
    let mut byte = [0u8; 1];
    let read = emu
        .handle_management
        .get_mut_file_handle(handle)
        .and_then(|fh| fh.read(&mut byte).ok());
    log_red!(
        emu,
        "{}!{} 0x{:x} = {:?}",
        runtime.library(),
        api,
        file,
        read.map(|_| byte[0])
    );
    match read {
        Some(1) => call.set_return(emu, byte[0] as u64),
        Some(0) => {
            with_stream(emu, runtime, file, |stream| stream.eof = true);
            call.set_return(emu, EOF);
        }
        _ => {
            with_stream(emu, runtime, file, |stream| stream.error = true);
            set_errno(emu, runtime, EIO);
            call.set_return(emu, EOF);
        }
    }
}

// The emulated standard input is empty; these exports have no FILE* argument.
pub(super) fn read_stdin(emu: &mut emu::Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    log_red!(emu, "{}!{} => EOF", runtime.library(), api);
    call.set_return(emu, EOF);
}
