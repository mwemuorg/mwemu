use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let value0 = match emu.get_operand_value(ins, 0, true) {
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

    // set the dest to the sum
    if !emu.set_operand_value(ins, 0, res) {
        return false;
    }

    // and then set the src to dest
    // doing in reverse can cause some error for example
    // xadd  [rsp+r9*4+16h],r9
    // which if we assign r9 first then resolve for address make the memory operations fail
    if !emu.set_operand_value(ins, 1, value0) {
        return false;
    }
    true
}
