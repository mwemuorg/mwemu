use crate::emu;

pub(super) fn CoUninitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!CoUninitialize");
    emu.regs_mut().rax = 0;
}
