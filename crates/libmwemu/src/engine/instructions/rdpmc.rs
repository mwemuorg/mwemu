use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// `rdpmc`: edx:eax = performance counter[ecx]. Counters read as the
/// instruction counter, which is monotonic like the real thing.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let v = emu.pos;
    emu.regs_mut().rax = v & 0xffff_ffff;
    emu.regs_mut().rdx = v >> 32;
    true
}
