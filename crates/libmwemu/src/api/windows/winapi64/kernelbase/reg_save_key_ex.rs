use crate::emu;
use crate::winapi::common::strings;

pub fn RegSaveKeyExA(emu: &mut emu::Emu) {
    reg_save_key_ex(emu, "RegSaveKeyExA", false);
}

fn reg_save_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let file_ptr = emu.regs().rdx;
    let _security = emu.regs().r8;
    let flags = emu.regs().r9;
    let filename = if file_ptr != 0 {
        strings::read_text(emu, file_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernelbase!{api} hkey:0x{:x} '{}' flags:0x{:x}",
        hkey,
        filename,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
}

pub fn RegSaveKeyExW(emu: &mut emu::Emu) {
    reg_save_key_ex(emu, "RegSaveKeyExW", true);
}
