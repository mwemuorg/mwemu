use crate::emu;

pub fn ApplicationRecoveryInProgress(emu: &mut emu::Emu) {
    let p_cancelled = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!ApplicationRecoveryInProgress pCancelled:0x{:x}",
        p_cancelled
    );

    if p_cancelled != 0 {
        emu.maps.write_dword(p_cancelled, 0);
    }
    emu.regs_mut().rax = 0;
}
