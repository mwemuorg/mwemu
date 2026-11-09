use crate::emu;

pub fn GetThreadPreferredUILanguages(emu: &mut emu::Emu) {
    let flags = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let num_langs_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let buff = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let out_sz = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

    emu.maps.write_dword(num_langs_ptr, 0);
    log_red!(emu, "kernel32!GetThreadPreferredUILanguages");

    emu.maps.write_dword(out_sz, 0);
    emu.maps.write_dword(num_langs_ptr, 0);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1;
}
