use crate::emu;
use crate::winapi::common::strings;

pub fn GetNamedSecurityInfoExA(emu: &mut emu::Emu) {
    get_named_security_info_ex(emu, "GetNamedSecurityInfoExA", false);
}

fn get_named_security_info_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
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

pub fn GetNamedSecurityInfoExW(emu: &mut emu::Emu) {
    get_named_security_info_ex(emu, "GetNamedSecurityInfoExW", true);
}
