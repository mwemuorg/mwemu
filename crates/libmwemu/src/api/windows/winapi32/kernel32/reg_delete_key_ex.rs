use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteKeyExA(emu: &mut emu::Emu) {
    reg_delete_key_ex(emu, "RegDeleteKeyExA", false);
}

fn reg_delete_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn RegDeleteKeyExW(emu: &mut emu::Emu) {
    reg_delete_key_ex(emu, "RegDeleteKeyExW", true);
}
