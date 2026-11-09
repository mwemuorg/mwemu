use crate::emu;

pub fn AppPolicyGetProcessTerminationMethod(emu: &mut emu::Emu) {
    let policy_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!AppPolicyGetProcessTerminationMethod (stub)"
    );
    if policy_ptr != 0 {
        emu.maps.write_dword(policy_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
