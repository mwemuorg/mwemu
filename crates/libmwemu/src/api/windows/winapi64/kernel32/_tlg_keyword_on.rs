use crate::emu;

pub fn _tlgKeywordOn(emu: &mut emu::Emu) {
    let provider = emu.regs().rcx;
    let keyword = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!_tlgKeywordOn provider:0x{:x} keyword:0x{:x}",
        provider,
        keyword
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
