use crate::emu;

pub fn PostQueuedCompletionStatus(emu: &mut emu::Emu) {
    let h_port = emu.regs().rcx;
    let num_bytes = emu.regs().rdx as u32;
    let comp_key = emu.regs().r8;
    let p_overlapped = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!PostQueuedCompletionStatus hPort:0x{:x} bytes:{} key:0x{:x} pOvl:0x{:x}",
        h_port,
        num_bytes,
        comp_key,
        p_overlapped
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
