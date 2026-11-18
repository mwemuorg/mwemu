use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WaitForMultipleObjectsEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let count = api_try!(emu, call.arg(emu, 0));
    let handles = api_try!(emu, call.arg(emu, 1));
    let wait_all = api_try!(emu, call.arg(emu, 2));
    let ms = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "kernel32!WaitForMultipleObjectsEx count:{} wait_all:{} ms:{}",
        count,
        wait_all,
        ms
    );
    emu.set_last_error(0);
    call.set_return(emu, 0); // WAIT_OBJECT_0
}
