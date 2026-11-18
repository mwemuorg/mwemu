use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WaitForMultipleObjects(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let count = api_try!(emu, call.arg(emu, 0));
    let _handles = api_try!(emu, call.arg(emu, 1));
    let _wait_all = api_try!(emu, call.arg(emu, 2));
    let ms = api_try!(emu, call.arg(emu, 3));
    helper::advance_tick(emu, ms);
    log_red!(
        emu,
        "kernelbase!WaitForMultipleObjects count:{} ms:{} => WAIT_OBJECT_0",
        count,
        ms
    );
    emu.set_last_error(0);
    call.set_return(emu, 0); // WAIT_OBJECT_0
}
