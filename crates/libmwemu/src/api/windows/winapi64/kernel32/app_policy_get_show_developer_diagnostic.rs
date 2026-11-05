use crate::emu;

pub fn AppPolicyGetShowDeveloperDiagnostic(emu: &mut emu::Emu) {
    let token = emu.regs().rcx;
    let p_policy = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!AppPolicyGetShowDeveloperDiagnostic token:0x{:x} pPolicy:0x{:x}",
        token,
        p_policy
    );

    if p_policy != 0 {
        emu.maps.write_dword(p_policy, 0);
    }
    emu.regs_mut().rax = 0;
}
