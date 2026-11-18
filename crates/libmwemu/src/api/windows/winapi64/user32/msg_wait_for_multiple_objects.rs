use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn MsgWaitForMultipleObjects(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let count = api_try!(emu, call.arg_u32(emu, 0));
    let handles = api_try!(emu, call.arg(emu, 1));
    let wait_all = api_try!(emu, call.arg(emu, 2)) != 0;
    let ms = api_try!(emu, call.arg_u32(emu, 3));
    let wake_mask = api_try!(emu, call.arg_u32(emu, 4));
    helper::advance_tick(emu, ms as u64);
    log_red!(
        emu,
        "user32!MsgWaitForMultipleObjects count:{} all:{} ms:{} mask:0x{:x}",
        count,
        wait_all,
        ms,
        wake_mask
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
