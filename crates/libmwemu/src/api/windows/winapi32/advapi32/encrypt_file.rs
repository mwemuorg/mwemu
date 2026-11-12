use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn EncryptFileA(emu: &mut emu::Emu) {
    encrypt_file(emu, "EncryptFileA", false);
}

fn encrypt_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    stdcall_ret(emu, 1);
    emu.regs_mut().rax = 1;
}

pub fn EncryptFileW(emu: &mut emu::Emu) {
    encrypt_file(emu, "EncryptFileW", true);
}
