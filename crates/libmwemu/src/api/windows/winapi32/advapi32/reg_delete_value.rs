use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteValueA(emu: &mut emu::Emu) {
    reg_delete_value(emu, "RegDeleteValueA", false);
}

fn reg_delete_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteValueW(emu: &mut emu::Emu) {
    reg_delete_value(emu, "RegDeleteValueW", true);
}
