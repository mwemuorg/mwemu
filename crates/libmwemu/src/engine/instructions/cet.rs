use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CET (Control-flow Enforcement Technology) shadow-stack instructions.
// On hardware without CET enabled (our case — we do not model shadow stacks),
// these instructions are defined to be NOPs, with the destination register
// (if any) preserving its previous value. Real ntdll relies on this exact
// behaviour: e.g. `xor edx, edx; rdsspq rdx; test rdx, rdx; je ...`
// expects rdx to remain 0 on no-CET CPUs.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    true
}
