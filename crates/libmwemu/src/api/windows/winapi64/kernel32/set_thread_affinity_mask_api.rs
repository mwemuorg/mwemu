use crate::emu;

pub fn SetThreadAffinityMask_api(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let mask = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!SetThreadAffinityMask h:0x{:x} mask:0x{:x}",
        handle,
        mask
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous mask
}
