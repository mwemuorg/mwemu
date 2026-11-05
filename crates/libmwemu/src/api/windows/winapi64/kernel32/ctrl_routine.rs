use crate::emu;

pub fn CtrlRoutine(emu: &mut emu::Emu) {
    let ctrl_type = emu.regs().rcx;
    log_red!(emu, "kernel32!CtrlRoutine ctrlType:{}", ctrl_type);
    emu.set_last_error(0);
}
