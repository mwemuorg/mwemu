use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn EnumServicesStatusA(emu: &mut emu::Emu) {
    enum_services_status(emu, "EnumServicesStatusA", false);
}

fn enum_services_status(emu: &mut emu::Emu, api: &str, wide: bool) {
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
    log_red!(
        emu,
        "advapi32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn EnumServicesStatusW(emu: &mut emu::Emu) {
    enum_services_status(emu, "EnumServicesStatusW", true);
}
