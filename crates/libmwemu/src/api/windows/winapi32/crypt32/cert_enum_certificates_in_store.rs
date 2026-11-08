use crate::emu;

pub fn CertEnumCertificatesInStore(emu: &mut emu::Emu) {
    let store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let prev_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CertEnumCertificatesInStore store:0x{:x} prev:0x{:x}",
        store,
        prev_ctx
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
