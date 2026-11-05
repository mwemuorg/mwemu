use crate::emu;

pub fn AddScopedPolicyIDAce(emu: &mut emu::Emu) {
    let acl = emu.regs().rcx;
    let ace_revision = emu.regs().rdx as u32;
    let ace_flags = emu.regs().r8 as u32;
    let access_mask = emu.regs().r9 as u32;
    log_red!(
        emu,
        "kernel32!AddScopedPolicyIDAce acl:0x{:x} rev:{} flags:0x{:x} mask:0x{:x}",
        acl,
        ace_revision,
        ace_flags,
        access_mask
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
