use crate::emu;
pub fn IsNormalizedString(emu: &mut emu::Emu) {
    let norm_form = emu.regs().rcx;
    let lp_string = emu.regs().rdx;
    let cw_length = emu.regs().r8 as i64;
    log_red!(
        emu,
        "kernel32!IsNormalizedString form:{} str:0x{:x} len:{}",
        norm_form,
        lp_string,
        cw_length
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
