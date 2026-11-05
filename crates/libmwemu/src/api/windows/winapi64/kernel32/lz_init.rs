use crate::emu;

pub fn LZInit(emu: &mut emu::Emu) {
    let hf_source = emu.regs().rcx;
    log_red!(emu, "kernel32!LZInit hfSource:0x{:x}", hf_source);
    emu.set_last_error(0);
    emu.regs_mut().rax = hf_source;
}
