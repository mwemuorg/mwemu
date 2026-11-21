use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{set_errno, stream_handle};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn ftell(emu: &mut emu::Emu, runtime: Runtime, api: &str, long: bool) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    let pos = stream_handle(emu, runtime, file).and_then(|handle| {
        emu.handle_management
            .get_mut_file_handle(handle)
            .map(|fh| fh.get_position())
    });
    log_red!(
        emu,
        "{}!{} 0x{:x} = {:?}",
        runtime.library(),
        api,
        file,
        pos
    );
    match pos {
        Some(pos) if long && pos > i32::MAX as u64 => {
            set_errno(emu, runtime, EINVAL);
            call.set_return(emu, EOF);
        }
        Some(pos) if long => call.set_return(emu, pos),
        Some(pos) => call.set_return_u64(emu, pos),
        None => {
            set_errno(emu, runtime, EINVAL);
            if long {
                call.set_return(emu, EOF);
            } else {
                call.set_return_u64(emu, u64::MAX);
            }
        }
    }
}
