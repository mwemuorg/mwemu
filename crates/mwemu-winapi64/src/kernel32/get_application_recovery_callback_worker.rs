use crate::emu;

pub fn GetApplicationRecoveryCallbackWorker(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let callback_ptr = emu.regs().rdx;
    let param_ptr = emu.regs().r8;
    let interval_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetApplicationRecoveryCallbackWorker h:0x{:x}",
        h_process
    );
    if callback_ptr != 0 {
        emu.maps.write_qword(callback_ptr, 0);
    }
    emu.regs_mut().rax = 0x80004005u64;
}
