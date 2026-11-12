use crate::emu;
use crate::winapi::common::strings;

pub fn EncryptFileA(emu: &mut emu::Emu) {
    encrypt_file(emu, "EncryptFileA", false);
}

fn encrypt_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn EncryptFileW(emu: &mut emu::Emu) {
    encrypt_file(emu, "EncryptFileW", true);
}
