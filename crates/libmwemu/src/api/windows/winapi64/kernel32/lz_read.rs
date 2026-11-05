use crate::emu;

pub fn LZRead(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let lp_buffer = emu.regs().rdx;
    let cb_read = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!LZRead hFile:0x{:x} buf:0x{:x} count:{}",
        h_file,
        lp_buffer,
        cb_read
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
