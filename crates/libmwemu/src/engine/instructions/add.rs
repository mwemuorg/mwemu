use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 2);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let res: u64 = match emu.get_operand_sz(ins, 1) {
        64 => emu
            .flags_overwrite_mut()
            .add64(value0, value1, false, false),
        32 => emu.flags_overwrite_mut().add32(
            (value0 & 0xffffffff) as u32,
            (value1 & 0xffffffff) as u32,
            false,
            false,
        ),
        16 => emu.flags_overwrite_mut().add16(
            (value0 & 0xffff) as u16,
            (value1 & 0xffff) as u16,
            false,
            false,
        ),
        8 => emu.flags_overwrite_mut().add8(
            (value0 & 0xff) as u8,
            (value1 & 0xff) as u8,
            false,
            false,
        ),
        _ => unreachable!("weird size"),
    };

    if !emu.set_operand_value(ins, 0, res) {
        return false;
    }
    true
}
