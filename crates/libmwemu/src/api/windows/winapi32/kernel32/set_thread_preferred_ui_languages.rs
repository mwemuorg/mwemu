use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetThreadPreferredUILanguages(emu: &mut emu::Emu) {
    let flags = arg(emu, 0) as u64 as u32;
    let languages = arg(emu, 1) as u64;
    let num_languages = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!SetThreadPreferredUILanguages flags:0x{:x} langs:0x{:x} pNum:0x{:x}",
        flags,
        languages,
        num_languages
    );
    if num_languages != 0 {
        emu.maps.write_dword(num_languages, 1);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
