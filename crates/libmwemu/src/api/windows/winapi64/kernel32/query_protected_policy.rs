use crate::emu;

pub fn QueryProtectedPolicy(emu: &mut emu::Emu) {
    let policy_guid = emu.regs().rcx;
    let p_policy_value = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!QueryProtectedPolicy guid:0x{:x} pValue:0x{:x}",
        policy_guid,
        p_policy_value
    );
    if p_policy_value != 0 {
        emu.maps.write_qword(p_policy_value, 0);
    }
    emu.regs_mut().rax = 1;
}
