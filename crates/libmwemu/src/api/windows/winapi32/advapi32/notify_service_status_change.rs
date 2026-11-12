use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn NotifyServiceStatusChange(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "advapi32!NotifyServiceStatusChange 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}

pub fn NotifyServiceStatusChangeA(emu: &mut emu::Emu) {
    notify_service_status_change(emu, "NotifyServiceStatusChangeA", false);
}

fn notify_service_status_change(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    stdcall_ret(emu, 3);
    emu.regs_mut().rax = 1;
}

pub fn NotifyServiceStatusChangeW(emu: &mut emu::Emu) {
    notify_service_status_change(emu, "NotifyServiceStatusChangeW", true);
}
