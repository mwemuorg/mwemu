use super::with_stream;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn clearerr(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!clearerr 0x{:x}", file);
    with_stream(file, |s| {
        s.eof = false;
        s.error = false;
    });
}
