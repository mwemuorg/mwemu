use crate::emu;

pub fn GetProcessIdOfThread(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetProcessIdOfThread h:0x{:x} => 0x123",
        h_thread
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x123;
}
