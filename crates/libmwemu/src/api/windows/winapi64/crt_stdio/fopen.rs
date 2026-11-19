use super::open;
use super::read_text;
use super::set_errno;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn fopen(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path = read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let mode = read_text(emu, api_try!(emu, call.arg(emu, 1)), wide);
    let result = open(emu, &path, &mode);
    log_red!(emu, "wincrt!{} '{}' '{}' = {:x?}", api, path, mode, result);
    match result {
        Ok(stream) => call.set_return(emu, stream),
        Err(errno) => {
            set_errno(emu, errno);
            call.set_return(emu, 0);
        }
    }
}
