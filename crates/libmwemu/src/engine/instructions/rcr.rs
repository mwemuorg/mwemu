use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 1 || ins.op_count() == 2);

    let result: u64;
    let sz = emu.get_operand_sz(ins, 0);

    if ins.op_count() == 1 {
        // 1 param
        let value0 = match emu.get_operand_value(ins, 0, true) {
            Some(v) => v,
            None => return false,
        };

        result = emu.flags_mut().rcr(value0, 1, sz);
    } else {
        // 2 params
        let value0 = match emu.get_operand_value(ins, 0, true) {
            Some(v) => v,
            None => return false,
        };

        let value1 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };

        result = emu.flags_mut().rcr(value0, value1, sz);
    }

    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
