use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let ms = api_try!(emu, call.arg(emu, 1));
    helper::advance_tick(emu, ms);
    log_red!(
        emu,
        "kernelbase!WaitForSingleObject handle:0x{:x} ms:{} => WAIT_OBJECT_0",
        handle,
        ms
    );
    emu.set_last_error(0);
    call.set_return(emu, 0); // WAIT_OBJECT_0
}
