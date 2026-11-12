use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SetNamedSecurityInfoExA(emu: &mut emu::Emu) {
    set_named_security_info_ex(emu, "SetNamedSecurityInfoExA", false);
}

fn set_named_security_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn SetNamedSecurityInfoExW(emu: &mut emu::Emu) {
    set_named_security_info_ex(emu, "SetNamedSecurityInfoExW", true);
}
