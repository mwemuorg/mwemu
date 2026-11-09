use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let imm = try_operand!(emu.get_operand_value(ins, 2, true)) as u8;
    let mode = if imm & 4 != 0 { 0 } else { imm & 3 };
    let mut result = 0u128;

    for i in 0..2 {
        let shift = i * 64;
        let x = f64::from_bits(((src >> shift) & 0xffff_ffff_ffff_ffff) as _);
        let r = match mode {
            0 => x.round_ties_even(),
            1 => x.floor(),
            2 => x.ceil(),
            _ => x.trunc(),
        };
        result |= ((r.to_bits() as u128) & 0xffff_ffff_ffff_ffff_u128) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
