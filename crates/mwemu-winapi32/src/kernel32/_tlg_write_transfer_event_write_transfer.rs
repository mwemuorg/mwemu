use crate::emu;

pub fn _tlgWriteTransfer_EventWriteTransfer(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!_tlgWriteTransfer_EventWriteTransfer");
    emu.regs_mut().rax = 0;
}
