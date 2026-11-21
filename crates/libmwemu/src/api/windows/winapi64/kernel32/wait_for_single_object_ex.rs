use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WaitForSingleObjectEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let ms = api_try!(emu, call.arg(emu, 1));
    let alertable = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "kernel32!WaitForSingleObjectEx h:0x{:x} ms:{} alert:{}",
        handle,
        ms,
        alertable
    );
    emu.set_last_error(0);
    call.set_return(emu, 0); // WAIT_OBJECT_0
}
