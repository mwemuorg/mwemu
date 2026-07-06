use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 1);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let res = match emu.get_operand_sz(ins, 0) {
        64 => emu.flags_overwrite_mut().dec64(value0),
        32 => emu.flags_overwrite_mut().dec32(value0),
        16 => emu.flags_overwrite_mut().dec16(value0),
        8 => emu.flags_overwrite_mut().dec8(value0),
        _ => panic!("weird size"),
    };

    if !emu.set_operand_value(ins, 0, res) {
        return false;
    }
    true
}
