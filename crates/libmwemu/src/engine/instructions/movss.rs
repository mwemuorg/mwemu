use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if ins.op_count() > 2 {
        unimplemented!("Movss with 3 operands is not implemented yet");
    }

    assert!(ins.op_count() == 2);

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    if sz1 == 128 {
        let val = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };

        let vf32: f32 = f32::from_bits((val & 0xFFFFFFFF) as u32);
        let result: u32 = vf32.to_bits();

        if !emu.set_operand_value(ins, 0, result as u64) {
            return false;
        }
    } else if sz0 == 128 && sz1 < 128 {
        let val = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };

        let value1_f32: f32 = f32::from_bits(val as u32);
        let result: u32 = value1_f32.to_bits();
        let xmm_value: u128 = result as u128;

        emu.set_operand_xmm_value_128(ins, 0, xmm_value);
    } else {
        unimplemented!("Movss unimplemented operation");
    }
    true
}
