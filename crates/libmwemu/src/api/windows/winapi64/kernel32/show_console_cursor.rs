use crate::emu;

pub fn ShowConsoleCursor(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let show = emu.regs().rdx != 0;

    log_red!(
        emu,
        "kernel32!ShowConsoleCursor h:0x{:x} show:{}",
        handle,
        show
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
