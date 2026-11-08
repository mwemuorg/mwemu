use crate::emu;

pub fn WakeConditionVariable(emu: &mut emu::Emu, api: &str) {
    let cv = emu.regs().rcx;
    log_red!(emu, "kernel32!{} cv:0x{:x}", api, cv);
}
