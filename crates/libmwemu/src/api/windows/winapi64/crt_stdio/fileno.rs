use super::EINVAL;
use super::EOF;
use super::set_errno;
use super::with_stream;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn fileno(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    let fd = with_stream(file, |s| s.fd);
    log_red!(emu, "wincrt!_fileno 0x{:x} = {:?}", file, fd);
    match fd {
        Some(fd) => call.set_return(emu, fd as u64),
        None => {
            set_errno(emu, EINVAL);
            call.set_return(emu, EOF);
        }
    }
}
