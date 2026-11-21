use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(emu: &mut emu::Emu) {
    access_check_by_type_result_list_and_audit_alarm_by_handle(
        emu,
        "AccessCheckByTypeResultListAndAuditAlarmByHandleA",
        false,
    );
}

fn access_check_by_type_result_list_and_audit_alarm_by_handle(
    emu: &mut emu::Emu,
    api: &str,
    wide: bool,
) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 17);
    call.set_return(emu, 1);
}

pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(emu: &mut emu::Emu) {
    access_check_by_type_result_list_and_audit_alarm_by_handle(
        emu,
        "AccessCheckByTypeResultListAndAuditAlarmByHandleW",
        true,
    );
}
