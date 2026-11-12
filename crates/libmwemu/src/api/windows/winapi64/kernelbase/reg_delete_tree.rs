use crate::emu;
use crate::winapi::common::strings;

pub fn RegDeleteTreeA(emu: &mut emu::Emu) {
    reg_delete_tree(emu, "RegDeleteTreeA", false);
}

fn reg_delete_tree(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let subkey = if subkey_ptr != 0 {
        strings::read_text(emu, subkey_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!{api} hkey:0x{:x} '{}'", hkey, subkey);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}

pub fn RegDeleteTreeW(emu: &mut emu::Emu) {
    reg_delete_tree(emu, "RegDeleteTreeW", true);
}
