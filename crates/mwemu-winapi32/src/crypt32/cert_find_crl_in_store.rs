use crate::emu;

pub fn CertFindCRLInStore(emu: &mut emu::Emu) {
    let h_cert_store = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_find_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_find_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let pv_find_para = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let p_prev_crl_ctx = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CertFindCRLInStore 0x{:x}", h_cert_store);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
