use crate::emu;

pub fn CertVerifyCertificateChainPolicy(emu: &mut emu::Emu) {
    let policy_oid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let chain_ctx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let policy_para = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let policy_status = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    if policy_status != 0 {
        emu.maps.write_dword(policy_status, 0); // cbSize
        emu.maps.write_dword(policy_status + 4, 0); // dwError = 0 (valid)
    }
    log_red!(
        emu,
        "crypt32!CertVerifyCertificateChainPolicy chain:0x{:x} => valid",
        chain_ctx
    );
    emu.regs_mut().rax = 1;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
