use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{open, read_text, set_errno};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

use crate::exception::types::ExceptionType;
use crate::winapi::common::memory;

pub(super) fn fopen_s(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool) {
    let call = WinApi(Cdecl);
    let output = api_try!(emu, call.arg(emu, 0));
    let path = api_try!(emu, call.arg(emu, 1));
    let mode = api_try!(emu, call.arg(emu, 2));
    if output == 0 {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, EINVAL as u64);
        return;
    }
    if !memory::writable_pointer(emu, output) {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    let path = read_text(emu, path, wide);
    let mode = read_text(emu, mode, wide);
    let result = path
        .as_deref()
        .zip(mode.as_deref())
        .ok_or(EINVAL)
        .and_then(|(path, mode)| open(emu, runtime, path, mode));
    log_red!(
        emu,
        "{}!{api} output={output:#x} path={path:?} mode={mode:?} => {result:?}",
        runtime.library()
    );
    let (stream, error) = match result {
        Ok(stream) => (stream, 0),
        Err(error) => {
            set_errno(emu, runtime, error);
            (0, error)
        }
    };
    let bytes = stream.to_le_bytes();
    if !emu
        .maps
        .write_bytes(output, &bytes[..memory::pointer_size(emu) as usize])
    {
        emu.exception(ExceptionType::BadAddressDereferencing);
        return;
    }
    call.set_return(emu, error as u64);
}
