use crate::emu;

pub fn GetUmsSystemThreadInformation(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let info_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetUmsSystemThreadInformation h:0x{:x} ptr:0x{:x}",
        h_thread,
        info_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
