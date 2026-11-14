use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

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
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'  0x{:x}", s, arg2);
    stdcall_ret(emu, 17);
    emu.regs_mut().rax = 1;
}

pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(emu: &mut emu::Emu) {
    access_check_by_type_result_list_and_audit_alarm_by_handle(
        emu,
        "AccessCheckByTypeResultListAndAuditAlarmByHandleW",
        true,
    );
}
