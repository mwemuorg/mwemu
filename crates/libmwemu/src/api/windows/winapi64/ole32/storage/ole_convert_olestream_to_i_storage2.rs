use crate::emu;

pub fn OleConvertOLESTREAMToIStorage2(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "ole32!OleConvertOLESTREAMToIStorage2 arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1 {}
}
