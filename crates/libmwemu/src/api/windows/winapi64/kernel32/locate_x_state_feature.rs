use crate::emu;

pub fn LocateXStateFeature(emu: &mut emu::Emu) {
    let context = emu.regs().rcx;
    let feature_id = emu.regs().rdx;
    let p_length = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!LocateXStateFeature ctx:0x{:x} feature:{} pLen:0x{:x}",
        context,
        feature_id,
        p_length
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
