use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CertVerifyCertificateChainPolicy(emu: &mut emu::Emu) {
    let policy_oid = WinApiAbi::arg(emu, 0);
    let chain_ctx = WinApiAbi::arg(emu, 1);
    let policy_para = WinApiAbi::arg(emu, 2);
    let policy_status = WinApiAbi::arg(emu, 3);
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
