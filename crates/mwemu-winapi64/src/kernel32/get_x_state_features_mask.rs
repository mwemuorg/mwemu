use crate::emu;

pub fn GetXStateFeaturesMask(emu: &mut emu::Emu) {
    let context_ptr = emu.regs().rcx;
    let mask_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetXStateFeaturesMask ctx:0x{:x} mask:0x{:x}",
        context_ptr,
        mask_ptr
    );
    if mask_ptr != 0 {
        emu.maps.write_qword(mask_ptr, 0x7);
    }
    emu.regs_mut().rax = 1;
}
