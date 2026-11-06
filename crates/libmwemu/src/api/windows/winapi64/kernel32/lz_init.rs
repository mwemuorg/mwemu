
use crate::emu;
use crate::winapi::helper;
pub fn LZInit(emu: &mut emu::Emu) {
    let hf_source = emu.regs().rcx;
    log_red!(emu, "kernel32!LZInit hfSource:0x{:x}", hf_source);
    let handle = helper::handler_create("LzInit");
    emu.set_last_error(0);
    emu.regs_mut().rax = hf_source;
}
