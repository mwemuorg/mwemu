use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn AccessCheckByTypeResultListAndAuditAlarmA(emu: &mut emu::Emu) {
    access_check_by_type_result_list_and_audit_alarm(
        emu,
        "AccessCheckByTypeResultListAndAuditAlarmA",
        false,
    );
}

fn access_check_by_type_result_list_and_audit_alarm(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let p4 = WinApiAbi::arg(emu, 4);
    let p5 = WinApiAbi::arg(emu, 5);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn AccessCheckByTypeResultListAndAuditAlarmW(emu: &mut emu::Emu) {
    access_check_by_type_result_list_and_audit_alarm(
        emu,
        "AccessCheckByTypeResultListAndAuditAlarmW",
        true,
    );
}
