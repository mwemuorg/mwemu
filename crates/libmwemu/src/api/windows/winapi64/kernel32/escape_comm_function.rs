use crate::emu;

pub fn EscapeCommFunction(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let func = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!EscapeCommFunction hFile:0x{:x} func:{}",
        h_file,
        func
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
