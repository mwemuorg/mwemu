use crate::emu;

pub fn CertVerifyCertificateChainPolicy(emu: &mut emu::Emu) {
    let _policy_oid = emu.regs().rcx;
    let _p_chain_context = emu.regs().rdx;
    let _p_policy_para = emu.regs().r8;
    let p_policy_status = emu.regs().r9;

    log_red!(
        emu,
        "crypt32!CertVerifyCertificateChainPolicy status_out:0x{:x}",
        p_policy_status
    );

    if p_policy_status != 0 {
        emu.maps.write_dword(p_policy_status, 4);
        emu.maps.write_dword(p_policy_status + 4, 0);
        emu.maps.write_dword(p_policy_status + 8, 0);
        emu.maps.write_dword(p_policy_status + 12, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
