use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn FindFirstChangeNotificationA(emu: &mut emu::Emu) {
    find_first_change_notification(emu, "FindFirstChangeNotificationA");
}

fn find_first_change_notification(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 3);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn FindFirstChangeNotificationW(emu: &mut emu::Emu) {
    find_first_change_notification(emu, "FindFirstChangeNotificationW");
}
