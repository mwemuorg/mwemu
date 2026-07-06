use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// MOVBE r, m / m, r : move data after swapping its byte order.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);

    let sz = emu.get_operand_sz(ins, 0);
    let val = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let swapped = match sz {
        16 => (val as u16).swap_bytes() as u64,
        32 => (val as u32).swap_bytes() as u64,
        64 => val.swap_bytes(),
        _ => return false,
    };

    emu.set_operand_value(ins, 0, swapped);
    true
}
