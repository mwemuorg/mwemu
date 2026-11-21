use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindFirstChangeNotificationA(emu: &mut emu::Emu) {
    find_first_change_notification(emu, "FindFirstChangeNotificationA", false);
}

fn find_first_change_notification(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    let hndl = helper::handler_create("change");
    log_red!(
        emu,
        "kernelbase!{api} '{}' p1:0x{:x} p2:0x{:x} => 0x{:x}",
        s,
        p1,
        p2,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}

pub fn FindFirstChangeNotificationW(emu: &mut emu::Emu) {
    find_first_change_notification(emu, "FindFirstChangeNotificationW", true);
}
