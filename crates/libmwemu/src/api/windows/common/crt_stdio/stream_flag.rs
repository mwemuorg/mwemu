use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{Stream, set_errno, with_stream};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn stream_flag(
    emu: &mut emu::Emu,
    runtime: Runtime,
    api: &str,
    flag: fn(&Stream) -> bool,
) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    let set = with_stream(emu, runtime, file, |s| flag(s));
    log_red!(
        emu,
        "{}!{} 0x{:x} = {:?}",
        runtime.library(),
        api,
        file,
        set
    );
    if set.is_none() {
        set_errno(emu, runtime, EINVAL);
    }
    call.set_return(emu, set.unwrap_or(false) as u64);
}
