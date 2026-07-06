use crate::emu;

pub fn IsValidCodePage(emu: &mut emu::Emu) {
    let codepage = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!IsValidCodePage error geting codepage param");

    log_red!(emu, "kernel32!IsValidCodePage {}", codepage);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
