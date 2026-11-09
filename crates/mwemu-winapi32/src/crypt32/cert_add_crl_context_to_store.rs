use crate::emu;

pub fn CertAddCRLContextToStore(emu: &mut emu::Emu) {
    let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_crl_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_add_disposition = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pp_store_ctx = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CertAddCRLContextToStore 0x{:x}", h_cert_store);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
