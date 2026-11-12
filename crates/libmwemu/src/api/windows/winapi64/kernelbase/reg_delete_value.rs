use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteValueA(emu: &mut emu::Emu) {
    reg_delete_value(emu, "RegDeleteValueA", false);
}

fn reg_delete_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "kernelbase!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn RegDeleteValueW(emu: &mut emu::Emu) {
    reg_delete_value(emu, "RegDeleteValueW", true);
}
