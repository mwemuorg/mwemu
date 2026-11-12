use crate::emu;
use crate::winapi::common::strings;

pub fn RegQueryMultipleValuesA(emu: &mut emu::Emu) {
    reg_query_multiple_values(emu, "RegQueryMultipleValuesA", false);
}

fn reg_query_multiple_values(emu: &mut emu::Emu, api: &str, wide: bool) {
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

pub fn RegQueryMultipleValuesW(emu: &mut emu::Emu) {
    reg_query_multiple_values(emu, "RegQueryMultipleValuesW", true);
}
