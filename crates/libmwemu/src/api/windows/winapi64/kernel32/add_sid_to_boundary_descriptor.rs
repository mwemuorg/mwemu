use crate::emu;

pub fn AddSIDToBoundaryDescriptor(emu: &mut emu::Emu) {
    let boundary_descriptor = emu.regs().rcx;
    let required_sid = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!AddSIDToBoundaryDescriptor desc:0x{:x} sid:0x{:x}",
        boundary_descriptor,
        required_sid
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
