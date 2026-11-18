use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateRemoteThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    let arg5 = api_try!(emu, call.arg(emu, 4));
    let arg6 = api_try!(emu, call.arg(emu, 5));
    let arg7 = api_try!(emu, call.arg(emu, 6));
    log_red!(
        emu,
        "kernelbase!CreateRemoteThread arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x} arg7:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7
    );
    let hndl = helper::handler_create("thread://CreateRemoteThread");
    call.set_return(emu, hndl);
}
