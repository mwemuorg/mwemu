use crate::emu;
use crate::winapi::common::strings;

pub fn RegUnLoadKeyA(emu: &mut emu::Emu) {
    reg_un_load_key(emu, "RegUnLoadKeyA", false);
}

fn reg_un_load_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}' p1:0x{:x}", s, p1);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn RegUnLoadKeyW(emu: &mut emu::Emu) {
    reg_un_load_key(emu, "RegUnLoadKeyW", true);
}
