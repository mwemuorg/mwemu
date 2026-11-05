use crate::emu;

pub fn GetThreadIOPendingFlag(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let io_pending_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetThreadIOPendingFlag h:0x{:x} ptr:0x{:x}",
        h_thread,
        io_pending_ptr
    );
    if io_pending_ptr != 0 {
        emu.maps.write_dword(io_pending_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
