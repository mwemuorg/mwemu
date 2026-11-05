use crate::emu;

pub fn FlushViewOfFile(emu: &mut emu::Emu) {
    let lp_base = emu.regs().rcx;
    let num_bytes = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!FlushViewOfFile base:0x{:x} bytes:{}",
        lp_base,
        num_bytes
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
