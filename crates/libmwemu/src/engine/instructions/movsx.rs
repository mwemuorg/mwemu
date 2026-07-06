use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 2);

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    assert!(
        !(sz1 != 8 || sz0 != 16 && sz0 != 32)
            || (sz0 == 32 && sz1 == 16)
            || (sz0 == 64 && sz1 == 32)
            || (sz0 == 64 && sz1 == 16)
            || (sz0 == 64 && sz1 == 8)
    );

    let mut result: u64 = 0;

    if sz0 == 16 {
        assert!(sz1 == 8);
        result = value1 as u8 as i8 as i16 as u16 as u64;
    } else if sz0 == 32 {
        if sz1 == 8 {
            result = value1 as u8 as i8 as i64 as u64;
        } else if sz1 == 16 {
            result = value1 as u16 as i16 as i32 as u32 as u64;
        }
    } else if sz0 == 64 {
        if sz1 == 8 {
            result = value1 as u8 as i8 as i64 as u64;
        } else if sz1 == 16 {
            result = value1 as u16 as i16 as i64 as u64;
        } else if sz1 == 32 {
            result = value1 as u32 as i32 as i64 as u64;
        }
    }

    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
