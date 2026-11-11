use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let (lo, hi) = avx::to_pair(try_operand!(emu.get_operand_ymm_value_256(ins, 1, true)));
    let imm = try_operand!(emu.get_operand_value(ins, 2, true));
    emu.set_operand_xmm_value_128(ins, 0, if imm & 1 != 0 { hi } else { lo })
}
