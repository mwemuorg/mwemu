use crate::emu;

pub fn WSCEnumNameSpaceProviders32(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!WSCEnumNameSpaceProviders32 arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
