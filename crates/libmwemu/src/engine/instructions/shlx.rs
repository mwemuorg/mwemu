use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// BMI2 SHLX dest, src, count : shift src by (count & size-1); flags unaffected.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let sz = emu.get_operand_sz(ins, 0) as u64;
    let src = emu.get_operand_value(ins, 1, true).unwrap_or(0);
    let cnt = emu.get_operand_value(ins, 2, true).unwrap_or(0);
    let shift = cnt & (sz - 1);
    let result = match sz {
        64 => src << shift,
        32 => ((src as u32) << shift) as u64,
        _ => return false,
    };
    emu.set_operand_value(ins, 0, result);
    true
}
