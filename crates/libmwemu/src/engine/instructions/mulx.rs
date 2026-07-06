use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// `MULX dest_hi, dest_lo, src` (BMI2) — unsigned multiply of `src` by the
/// implicit RDX/EDX, writing the low half to `dest_lo` and the high half to
/// `dest_hi`. Unlike `MUL`, it does **not** affect any flags. When both
/// destinations are the same register only the high half remains (we write
/// `dest_lo` first, then `dest_hi`, so the aliasing case is handled).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let src = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v,
        None => {
            log::trace!("mulx: error reading src operand");
            return false;
        }
    };

    if emu.get_operand_sz(ins, 0) == 64 {
        let factor = emu.regs().rdx as u128;
        let result = (src as u128).wrapping_mul(factor);
        emu.set_operand_value(ins, 1, result as u64); // low
        emu.set_operand_value(ins, 0, (result >> 64) as u64); // high
    } else {
        let factor = emu.regs().rdx & 0xffff_ffff; // edx
        let result = (src & 0xffff_ffff).wrapping_mul(factor); // fits in u64
        emu.set_operand_value(ins, 1, result & 0xffff_ffff); // low
        emu.set_operand_value(ins, 0, (result >> 32) & 0xffff_ffff); // high
    }
    // MULX intentionally leaves flags untouched.
    true
}
