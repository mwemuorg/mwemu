use crate::emu;

pub fn IsIoRingOpSupported(emu: &mut emu::Emu) {
    let io_ring = emu.regs().rcx;
    let op = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!IsIoRingOpSupported ring:0x{:x} op:{} => FALSE",
        io_ring,
        op
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
