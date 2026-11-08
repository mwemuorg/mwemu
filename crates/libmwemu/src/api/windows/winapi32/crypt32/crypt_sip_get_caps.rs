use crate::emu;

pub fn CryptSIPGetCaps(emu: &mut emu::Emu) {
    let p_subject_info = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_caps = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptSIPGetCaps p_subject_info:0x{:x} p_caps:0x{:x}",
        p_subject_info,
        p_caps
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
