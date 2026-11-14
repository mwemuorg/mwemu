use crate::emu;

pub fn CryptSIPCreateIndirectData(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "crypt32!CryptSIPCreateIndirectData 0x{:x} 0x{:x} 0x{:x}",
        arg1,
        arg2,
        arg3
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
