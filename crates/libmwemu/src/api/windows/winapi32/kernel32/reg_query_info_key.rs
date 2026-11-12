use crate::emu;
use crate::winapi::common::strings;

pub fn RegQueryInfoKeyA(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyA", false);
}

fn reg_query_info_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
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
    for _ in 0..12 {
        emu.stack_pop32(false);
    }
}

pub fn RegQueryInfoKeyW(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyW", true);
}
