use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn FindActCtxSectionStringA(emu: &mut Emu) {
    invoke(emu, false, "FindActCtxSectionStringA");
}

pub fn FindActCtxSectionStringW(emu: &mut Emu) {
    invoke(emu, true, "FindActCtxSectionStringW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let key = strings::read_text(emu, emu.regs().r9, wide);
    log_red!(
        emu,
        "kernel32!{} flags:0x{:x} section:{} key:'{}'",
        api,
        emu.regs().rcx,
        emu.regs().r8,
        key
    );
    // No activation-context sections are registered in this profile.
    emu.set_last_error(ERROR_SXS_KEY_NOT_FOUND);
    emu.regs_mut().rax = 0;
}
