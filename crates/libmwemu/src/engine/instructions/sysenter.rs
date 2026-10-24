use crate::emu::Emu;
use crate::ntapi;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.is_x64() {
        log::warn!("sysenter: ntapi64 not implemented yet");
        return false;
    } else {
        ntapi::ntapi32::gateway(emu.regs().get_eax(), emu.regs().get_edx(), emu);
    }
    true
}
