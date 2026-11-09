use crate::emu;

pub fn SetProtectedPolicy(emu: &mut emu::Emu) {
    let policy_guid = emu.regs().rcx;
    let policy_value = emu.regs().rdx;
    let p_old_value = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!SetProtectedPolicy guid:0x{:x} value:0x{:x} pOld:0x{:x}",
        policy_guid,
        policy_value,
        p_old_value
    );
    if p_old_value != 0 {
        emu.maps.write_qword(p_old_value, 0);
    }
    emu.regs_mut().rax = 1;
}
