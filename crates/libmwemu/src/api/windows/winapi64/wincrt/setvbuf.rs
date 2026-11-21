use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn setvbuf(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    // int setvbuf(FILE *stream, char *buf, int mode, size_t size): configures
    // stream buffering. We don't do real buffered I/O, so accept it and report
    // success (0).
    let stream = api_try!(emu, call.arg(emu, 0));
    let mode = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "wincrt!setvbuf stream: 0x{:x} mode: 0x{:x}",
        stream,
        mode
    );
    call.set_return(emu, 0);
}
