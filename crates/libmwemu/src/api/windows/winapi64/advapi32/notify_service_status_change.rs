use crate::emu;
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
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "advapi32!NotifyServiceStatusChangeA '{}'  p1:0x{:x}",
        s,
        p1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn NotifyServiceStatusChangeW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "advapi32!NotifyServiceStatusChangeW '{}'  p1:0x{:x}",
        s,
        p1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
