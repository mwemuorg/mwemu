use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let f = f32::from_bits((src & 0xffffffff) as u32);
    let r = if false {
        f.trunc()
    } else {
        f.round_ties_even()
    };
    let result: u64 = if emu.get_operand_sz(ins, 0) == 64 {
        if r.is_nan() || !(-9223372036854775808.0..9223372036854775808.0).contains(&r) {
            i64::MIN as u64
        } else {
            r as i64 as u64
        }
    } else if r.is_nan() || !(-2147483648.0..2147483648.0).contains(&r) {
        i32::MIN as u32 as u64
    } else {
        r as i32 as u32 as u64
    };
    emu.set_operand_value(ins, 0, result)
}
