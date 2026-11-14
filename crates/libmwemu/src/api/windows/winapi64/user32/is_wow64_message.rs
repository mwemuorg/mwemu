use crate::emu;

pub fn IsWow64Message(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "user32!IsWow64Message arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
