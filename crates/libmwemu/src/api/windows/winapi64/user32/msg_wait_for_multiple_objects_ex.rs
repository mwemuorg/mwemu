use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn MsgWaitForMultipleObjectsEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let count = api_try!(emu, call.arg_u32(emu, 0));
    let handles = api_try!(emu, call.arg(emu, 1));
    let ms = api_try!(emu, call.arg_u32(emu, 2));
    let wake_mask = api_try!(emu, call.arg_u32(emu, 3));
    let flags = api_try!(emu, call.arg_u32(emu, 4));
    helper::advance_tick(emu, ms as u64);
    log_red!(
        emu,
        "user32!MsgWaitForMultipleObjectsEx count:{} ms:{} mask:0x{:x} flags:0x{:x}",
        count,
        ms,
        wake_mask,
        flags
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
