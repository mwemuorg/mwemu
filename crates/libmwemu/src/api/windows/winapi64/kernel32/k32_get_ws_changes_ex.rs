use crate::emu;

pub fn K32GetWsChangesEx(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let p_info = emu.regs().rdx;
    let p_cb = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!K32GetWsChangesEx hProc:0x{:x} pInfo:0x{:x} pCb:0x{:x}",
        h_process,
        p_info,
        p_cb
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
