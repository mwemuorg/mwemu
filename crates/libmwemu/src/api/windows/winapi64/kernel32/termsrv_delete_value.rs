use crate::emu;

pub fn TermsrvDeleteValue(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!TermsrvDeleteValue param1:0x{:x} param2:0x{:x}",
        param1,
        param2
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
