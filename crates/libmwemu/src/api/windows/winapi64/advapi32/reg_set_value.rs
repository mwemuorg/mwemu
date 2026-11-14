use crate::emu;
use crate::winapi::common::strings;

pub fn RegSetValueA(emu: &mut emu::Emu) {
    reg_set_value(emu, "RegSetValueA", false);
}

fn reg_set_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let reg_type = emu.regs().r9;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' type:{}",
        hkey,
        name,
        reg_type
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegSetValueW(emu: &mut emu::Emu) {
    reg_set_value(emu, "RegSetValueW", true);
}
