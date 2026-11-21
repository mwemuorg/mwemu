use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn crt_beginthreadex(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let _security = api_try!(emu, call.arg(emu, 0));
    let _stack_size = api_try!(emu, call.arg(emu, 1));
    let start = api_try!(emu, call.arg(emu, 2));
    let arg = api_try!(emu, call.arg(emu, 3));
    let hndl = helper::handler_create(&format!("thread://0x{:x}", start));
    log_red!(
        emu,
        "wincrt!_beginthreadex start:0x{:x} arg:0x{:x} => 0x{:x}",
        start,
        arg,
        hndl
    );
    call.set_return(emu, hndl);
}
