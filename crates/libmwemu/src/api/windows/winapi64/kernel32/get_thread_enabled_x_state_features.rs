use crate::emu;

pub fn GetThreadEnabledXStateFeatures(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "kernel32!GetThreadEnabledXStateFeatures => 0x7 (x87+SSE+AVX)"
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x7;
}
