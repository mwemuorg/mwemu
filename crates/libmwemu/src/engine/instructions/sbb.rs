use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 2);

    let cf = emu.flag_cf();

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    let res: u64 = match sz {
        64 => emu.flags_overwrite_mut().sub64_borrow(value0, value1, cf),
        32 => emu
            .flags_overwrite_mut()
            .sub32_borrow(value0, value1 & 0xffff_ffff, cf),
        16 => emu
            .flags_overwrite_mut()
            .sub16_borrow(value0, value1 & 0xffff, cf),
        8 => emu
            .flags_overwrite_mut()
            .sub8_borrow(value0, value1 & 0xff, cf),
        _ => panic!("weird size"),
    };

    if !emu.set_operand_value(ins, 0, res) {
        return false;
    }
    true
}
