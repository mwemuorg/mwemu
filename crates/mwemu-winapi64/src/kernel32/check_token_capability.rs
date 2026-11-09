use crate::emu;

pub fn CheckTokenCapability(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rcx;
    let capability_sid = emu.regs().rdx;
    let has_capability = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!CheckTokenCapability token:0x{:x} sid:0x{:x} hasCap:0x{:x}",
        token_handle,
        capability_sid,
        has_capability
    );
    if has_capability != 0 {
        emu.maps.write_dword(has_capability, 0);
    }
    emu.regs_mut().rax = 1;
}
