use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));

    let sz1 = emu.get_operand_sz(ins, 1);

    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("cvtsi2sd: error reading source operand");
            return false;
        }
    };

    let dbl: f64 = if sz1 == 64 {
        (src as i64) as f64
    } else {
        (src as u32 as i32) as f64
    };

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let result = (dest & 0xFFFFFFFFFFFFFFFF_0000000000000000) | (dbl.to_bits() as u128);
    emu.set_operand_xmm_value_128(ins, 0, result)
}
