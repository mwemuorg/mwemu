use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn NotifyServiceStatusChange(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "advapi32!NotifyServiceStatusChange 0x{:x}", arg1);
    let out_ptr = api_try!(emu, call.arg(emu, 2));
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn NotifyServiceStatusChangeA(emu: &mut emu::Emu) {
    notify_service_status_change(emu, "NotifyServiceStatusChangeA", false);
}

fn notify_service_status_change(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn NotifyServiceStatusChangeW(emu: &mut emu::Emu) {
    notify_service_status_change(emu, "NotifyServiceStatusChangeW", true);
}
