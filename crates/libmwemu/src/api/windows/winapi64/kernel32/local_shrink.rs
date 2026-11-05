use crate::emu;

pub fn LocalShrink(emu: &mut emu::Emu) {
    let h_mem = emu.regs().rcx;
    let new_size = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!LocalShrink hMem:0x{:x} newSize:{}",
        h_mem,
        new_size
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xFFFF;
}
