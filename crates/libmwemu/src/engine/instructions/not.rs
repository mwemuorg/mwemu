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

    let val: u64;

    /*let mut ival = value0 as i32;
    ival = !ival;*/

    let sz = emu.get_operand_sz(ins, 0);
    match sz {
        64 => {
            let mut ival = value0 as i64;
            ival = !ival;
            val = ival as u64;
        }
        32 => {
            let mut ival = value0 as u32 as i32;
            ival = !ival;
            //val = value0 & 0xffffffff_00000000 | ival as u32 as u64;
            val = ival as u32 as u64;
        }
        16 => {
            let mut ival = value0 as u16 as i16;
            ival = !ival;
            val = value0 & 0xffffffff_ffff0000 | ival as u16 as u64;
        }
        8 => {
            let mut ival = value0 as u8 as i8;
            ival = !ival;
            val = value0 & 0xffffffff_ffffff00 | ival as u8 as u64;
        }
        _ => unimplemented!("weird"),
    }

    if !emu.set_operand_value(ins, 0, val) {
        return false;
    }
    true
}
