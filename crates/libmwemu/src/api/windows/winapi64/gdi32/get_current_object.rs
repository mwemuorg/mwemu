use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetCurrentObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let obj_type = api_try!(emu, call.arg(emu, 1));
    let tag = match obj_type {
        1 => "gdi32://pen",
        2 => "gdi32://brush",
        6 => "gdi32://font",
        7 => "gdi32://bitmap",
        _ => "",
    };
    let result: u64 = if tag.is_empty() {
        0
    } else {
        helper::handler_create(tag)
    };
    log_red!(
        emu,
        "gdi32!GetCurrentObject hdc:0x{:x} type:{} = 0x{:x}",
        hdc,
        obj_type,
        result
    );
    call.set_return(emu, result);
}
