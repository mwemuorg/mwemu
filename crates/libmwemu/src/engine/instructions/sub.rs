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

    /*
                if value0 == emu.regs().rsp {

    emu.show_instruction(color!("LightCyan"), &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins));

                    if emu.cfg.is_x64() {
                        if value1 % 8 == 0 {
                            emu.stack_lvl[emu.stack_lvl_idx] -= value1 as i32 / 8;
                        }
                    } else {
                        if value1 % 4 == 0 {
                            emu.stack_lvl[emu.stack_lvl_idx] -= value1 as i32 / 4;
                        }
                    }
                }*/

    let res: u64 = match emu.get_operand_sz(ins, 0) {
        64 => emu.flags_overwrite_mut().sub64(value0, value1),
        32 => emu.flags_overwrite_mut().sub32(value0, value1),
        16 => emu.flags_overwrite_mut().sub16(value0, value1),
        8 => emu.flags_overwrite_mut().sub8(value0, value1),
        _ => panic!("weird size"),
    };

    if !emu.set_operand_value(ins, 0, res) {
        return false;
    }
    true
}
