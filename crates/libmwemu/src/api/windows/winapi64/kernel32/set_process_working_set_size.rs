use crate::emu;

pub fn SetProcessWorkingSetSize(emu: &mut emu::Emu) {
    let hprocess = emu.regs().rcx;
    let min_size = emu.regs().rdx;
    let max_size = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!SetProcessWorkingSetSize proc:0x{:x} min:0x{:x} max:0x{:x}",
        hprocess,
        min_size,
        max_size
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
