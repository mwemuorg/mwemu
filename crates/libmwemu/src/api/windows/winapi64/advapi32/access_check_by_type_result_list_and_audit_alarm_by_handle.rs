use crate::emu;
use crate::winapi::common::strings;

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
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'  0x{:x}", s, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(emu: &mut emu::Emu) {
    access_check_by_type_result_list_and_audit_alarm_by_handle(
        emu,
        "AccessCheckByTypeResultListAndAuditAlarmByHandleW",
        true,
    );
}
