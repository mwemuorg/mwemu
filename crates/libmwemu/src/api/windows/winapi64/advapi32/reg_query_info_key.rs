use crate::emu;
use crate::winapi::common::strings;

pub fn RegQueryInfoKeyA(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyA", false);
}

fn reg_query_info_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let name = strings::read_text(emu, name_ptr, wide);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 2;
}

pub fn RegQueryInfoKeyW(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyW", true);
}
