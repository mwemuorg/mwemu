use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);

    let mut bit = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let value = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    bit %= sz as u64;

    let cf = get_bit!(value, bit);
    emu.flags_mut().f_cf = cf == 1;

    let mut result = value;
    set_bit!(result, bit, 0);

    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
