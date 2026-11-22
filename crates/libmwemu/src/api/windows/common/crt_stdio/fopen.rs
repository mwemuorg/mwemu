use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{open, read_text, set_errno};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn fopen(emu: &mut Emu, runtime: Runtime, api: &str, wide: bool) {
    let call = WinApi(Cdecl);
    let path = api_try!(emu, call.arg(emu, 0));
    let mode = api_try!(emu, call.arg(emu, 1));
    // The file-root model does not enforce interprocess CRT share modes.
    if matches!(api, "_fsopen" | "_wfsopen") {
        let _sharing = api_try!(emu, call.arg_i32(emu, 2));
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
        "{}!{api} path={path:?} mode={mode:?} => {result:?}",
        runtime.library()
    );
    match result {
        Ok(stream) => call.set_return(emu, stream),
        Err(error) => {
            set_errno(emu, runtime, error);
            call.set_return(emu, 0);
        }
    }
}
