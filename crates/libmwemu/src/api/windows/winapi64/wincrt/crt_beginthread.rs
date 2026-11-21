use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn crt_beginthread(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let start = api_try!(emu, call.arg(emu, 0));
    let stack_size = api_try!(emu, call.arg(emu, 1));
    let arg = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create(&format!("thread://0x{:x}", start));
    log_red!(
        emu,
        "wincrt!_beginthread start:0x{:x} stack:{} arg:0x{:x} => 0x{:x}",
        start,
        stack_size,
        arg,
        hndl
    );
    call.set_return(emu, hndl);
}
