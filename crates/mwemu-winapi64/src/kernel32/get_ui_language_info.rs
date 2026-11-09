use crate::emu;

pub fn GetUILanguageInfo(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let languages_ptr = emu.regs().rdx;
    let fallback_ptr = emu.regs().r8;
    let fallback_size_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetUILanguageInfo flags:0x{:x} lang:0x{:x}",
        flags,
        languages_ptr
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
