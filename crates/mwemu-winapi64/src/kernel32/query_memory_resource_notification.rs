use crate::emu;

pub fn QueryMemoryResourceNotification(emu: &mut emu::Emu) {
    let h_resource = emu.regs().rcx;
    let p_resource_state = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!QueryMemoryResourceNotification hRes:0x{:x} pState:0x{:x}",
        h_resource,
        p_resource_state
    );
    if p_resource_state != 0 {
        emu.maps.write_dword(p_resource_state, 0);
    }
    emu.regs_mut().rax = 1;
}
