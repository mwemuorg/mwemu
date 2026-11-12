use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegDeleteValueA(emu: &mut emu::Emu) {
    reg_delete_value(emu, "RegDeleteValueA", false);
}

fn reg_delete_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0) as u32;
    let name_ptr = WinApiAbi::arg(emu, 1);
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
