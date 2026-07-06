use crate::color;
use crate::emu::Emu;
use crate::engine::logic;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 1 || ins.op_count() == 2 || ins.op_count() == 3);

    if ins.op_count() == 1 {
        // 1 param

        let value0 = match emu.get_operand_value(ins, 0, true) {
            Some(v) => v,
            None => return false,
        };

        let pre_rax = emu.regs().rax;
        let pre_rdx = emu.regs().rdx;

        let sz = emu.get_operand_sz(ins, 0);
        match sz {
            64 => logic::imul64p1(emu, value0),
            32 => logic::imul32p1(emu, value0),
            16 => logic::imul16p1(emu, value0),
            8 => logic::imul8p1(emu, value0),
            _ => unimplemented!("wrong size"),
        }
    } else if ins.op_count() == 2 {
        // 2 params

        let value0 = match emu.get_operand_value(ins, 0, true) {
            Some(v) => v,
            None => return false,
        };

        let value1 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };

        let sz = emu.get_operand_sz(ins, 0);
        let result = match sz {
            64 => emu.flags_overwrite_mut().imul64p2(value0, value1),
            32 => emu.flags_overwrite_mut().imul32p2(value0, value1),
            16 => emu.flags_overwrite_mut().imul16p2(value0, value1),
            8 => emu.flags_overwrite_mut().imul8p2(value0, value1),
            _ => unimplemented!("wrong size"),
        };

        if !emu.set_operand_value(ins, 0, result) {
            return false;
        }
    } else {
        // 3 params

        let value1 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };

        let value2 = match emu.get_operand_value(ins, 2, true) {
            Some(v) => v,
            None => return false,
        };

        let sz = emu.get_operand_sz(ins, 0);
        let result = match sz {
            64 => emu.flags_overwrite_mut().imul64p2(value1, value2),
            32 => emu.flags_overwrite_mut().imul32p2(value1, value2),
            16 => emu.flags_overwrite_mut().imul16p2(value1, value2),
            8 => emu.flags_overwrite_mut().imul8p2(value1, value2),
            _ => unimplemented!("wrong size"),
        };

        if !emu.set_operand_value(ins, 0, result) {
            return false;
        }
    }
    true
}
