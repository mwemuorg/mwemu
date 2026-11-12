use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn NotifyServiceStatusChange(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(emu, "advapi32!NotifyServiceStatusChange 0x{:x}", arg1);
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn NotifyServiceStatusChangeA(emu: &mut emu::Emu) {
    notify_service_status_change(emu, "NotifyServiceStatusChangeA", false);
}

fn notify_service_status_change(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn NotifyServiceStatusChangeW(emu: &mut emu::Emu) {
    notify_service_status_change(emu, "NotifyServiceStatusChangeW", true);
}
