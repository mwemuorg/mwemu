
use crate::emu;
use crate::winapi::helper;
pub fn AddIntegrityLabelToBoundaryDescriptor(emu: &mut emu::Emu) {
    let boundary_descriptor = emu.regs().rcx;
    let integrity_label = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!AddIntegrityLabelToBoundaryDescriptor desc:0x{:x} label:0x{:x}",
        boundary_descriptor,
        integrity_label
    );
    let handle = helper::handler_create("AddIntegrityLabelToBoundaryDescriptor");
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
