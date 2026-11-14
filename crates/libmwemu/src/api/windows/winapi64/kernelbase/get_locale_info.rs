use crate::emu;
use crate::winapi::common::strings;

pub fn GetLocaleInfoA(emu: &mut emu::Emu) {
    get_locale_info(emu, "GetLocaleInfoA", false);
}

fn get_locale_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    let locale = emu.regs().rcx;
    let lctype = emu.regs().rdx;
    let buf_ptr = emu.regs().r8;
    let buf_size = emu.regs().r9;
    if buf_ptr != 0 && buf_size > 0 {
        strings::write_string(emu, buf_ptr, "en-US", wide);
    }
    log_red!(
        emu,
        "kernelbase!{api} locale:0x{:x} type:0x{:x}",
        locale,
        lctype
    );

    emu.regs_mut().rax = 6;
}

pub fn GetLocaleInfoW(emu: &mut emu::Emu) {
    get_locale_info(emu, "GetLocaleInfoW", true);
}
