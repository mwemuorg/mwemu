use crate::emu;

pub fn SetThreadPreferredUILanguages(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let languages = emu.regs().rdx;
    let num_languages = emu.regs().r8;
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
}
