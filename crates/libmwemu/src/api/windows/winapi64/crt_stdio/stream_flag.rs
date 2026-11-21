use super::EINVAL;
use super::Stream;
use super::set_errno;
use super::with_stream;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn stream_flag(emu: &mut emu::Emu, api: &str, flag: fn(&Stream) -> bool) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    let set = with_stream(file, |s| flag(s));
    log_red!(emu, "wincrt!{} 0x{:x} = {:?}", api, file, set);
    if set.is_none() {
        set_errno(emu, EINVAL);
    }
    call.set_return(emu, set.unwrap_or(false) as u64);
}
