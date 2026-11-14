use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// `wrmsr`: MSR[ecx] = edx:eax. A privileged operation; outside kernel mode it
/// is #GP, which is what the #UD path approximates.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if emu.kernel.as_ref().is_none_or(|k| k.ledger_only) {
        log::warn!("wrmsr outside kernel mode");
        return false;
    }
    let index = emu.regs().rcx as u32;
    let value = (emu.regs().rdx << 32) | (emu.regs().rax & 0xffff_ffff);
    emu.kernel_msr_write(index, value);
    true
}
