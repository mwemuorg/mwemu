use crate::emu;

pub fn GetThreadPreferredUILanguages(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let num_langs_ptr = emu.regs().rdx;
    let buff = emu.regs().r8;
    let out_sz = emu.regs().r9;

    emu.maps.write_dword(num_langs_ptr, 0);
    log_red!(emu, "kernel32!GetThreadPreferredUILanguages");

    emu.maps.write_dword(out_sz, 0);
    emu.maps.write_dword(num_langs_ptr, 0);

    emu.regs_mut().rax = 1;
}
