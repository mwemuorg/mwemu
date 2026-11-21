use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateUmsThreadContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    let arg3 = api_try!(emu, call.arg_u32(emu, 2));
    let arg4 = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "kernel32!CreateUmsThreadContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    call.cleanup_args(emu, 1);
    let hndl = helper::handler_create("kernel32://CreateUmsThreadContext");
    assert!(hndl < 0x00000001_00000000);
    call.set_return(emu, hndl);
}
