use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    debug_assert!(ins.op_count() == 2);
    debug_assert!(emu.get_operand_sz(ins, 0) == emu.get_operand_sz(ins, 1));

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    let (result1, result2): (u64, u64) = match sz {
        8 => {
            let r1 = (value0 & 0xff) | (value1 & 0xff);
            (r1, (value0 & 0xffffffffffffff00) + r1)
        }
        16 => {
            let r1 = (value0 & 0xffff) | (value1 & 0xffff);
            (r1, (value0 & 0xffffffffffff0000) + r1)
        }
        32 => {
            let r1 = (value0 & 0xffffffff) | (value1 & 0xffffffff);
            (r1, (value0 & 0xffffffff00000000) + r1)
        }
        64 => {
            let r1 = value0 | value1;
            (r1, r1)
        }
        _ => {
            log::warn!("or: unexpected operand size");
            return false;
        }
    };

    let bits = emu.get_operand_sz(ins, 0);
    emu.flags_overwrite_mut()
        .calc_logic_flags_lazy(result1, bits);

    if !emu.set_operand_value(ins, 0, result2) {
        return false;
    }
    true
}
