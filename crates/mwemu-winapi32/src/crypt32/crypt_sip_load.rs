use crate::emu;

pub fn CryptSIPLoad(emu: &mut emu::Emu) {
    let pg_subject = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_sip_dispatch = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptSIPLoad pg_subject:0x{:x} dw_flags:0x{:x} p_sip_dispatch:0x{:x}",
        pg_subject,
        dw_flags,
        p_sip_dispatch
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
