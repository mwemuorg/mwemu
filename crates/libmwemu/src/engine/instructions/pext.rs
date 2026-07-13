use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PEXT dest, src, mask: gather the bits of `src` selected by `mask` into the
// low-order bits of `dest`. Does not affect flags.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let val = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let mask = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v,
        None => return false,
    };

    let mut result = 0u64;
    let mut k = 0u32;
    for i in 0..64 {
        if (mask >> i) & 1 == 1 {
            if (val >> i) & 1 == 1 {
                result |= 1u64 << k;
            }
            k += 1;
        }
    }

    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
