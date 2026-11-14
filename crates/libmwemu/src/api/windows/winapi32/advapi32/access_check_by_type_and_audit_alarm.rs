use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn AccessCheckByTypeAndAuditAlarmA(emu: &mut emu::Emu) {
    access_check_by_type_and_audit_alarm(emu, "AccessCheckByTypeAndAuditAlarmA", false);
}

fn access_check_by_type_and_audit_alarm(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let p4 = WinApiAbi::arg(emu, 4);
    let p5 = WinApiAbi::arg(emu, 5);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    stdcall_ret(emu, 16);
    emu.regs_mut().rax = 1;
}

pub fn AccessCheckByTypeAndAuditAlarmW(emu: &mut emu::Emu) {
    access_check_by_type_and_audit_alarm(emu, "AccessCheckByTypeAndAuditAlarmW", true);
}
