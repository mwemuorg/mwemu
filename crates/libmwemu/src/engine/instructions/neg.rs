use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 1);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    let res = match sz {
        64 => emu.flags_mut().neg64(value0),
        32 => emu.flags_mut().neg32(value0),
        16 => emu.flags_mut().neg16(value0),
        8 => emu.flags_mut().neg8(value0),
        _ => panic!("weird size"),
    };

    emu.flags_mut().f_cf = value0 != 0;
    emu.flags_mut().f_af = ((res | value0) & 0x8) != 0;

    if !emu.set_operand_value(ins, 0, res) {
        return false;
    }
    true
}
