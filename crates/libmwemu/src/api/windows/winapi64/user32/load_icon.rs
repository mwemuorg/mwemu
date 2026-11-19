use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn LoadIconA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let lpiconname = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "user32!LoadIconA hInst:0x{:x} name:0x{:x}",
        hinstance,
        lpiconname
    );
    call.set_return(emu, helper::handler_create("user32://icon"));
}

pub fn LoadIconW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let icon_name = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "user32!LoadIconW hinst:0x{:x} icon:0x{:x}",
        hinstance,
        icon_name
    );
    let hndl = helper::handler_create("user32://LoadIconW");
    call.set_return(emu, hndl);
}
