use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// BLSI dest, src: dest = (-src) & src  (isolate the lowest set bit)
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz = emu.get_operand_sz(ins, 0);
    let mask: u64 = if sz >= 64 { u64::MAX } else { (1u64 << sz) - 1 };
    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v & mask,
        None => return false,
    };

    let result = src.wrapping_neg() & src & mask;

    // CF set when source is non-zero; OF cleared; SF/ZF reflect the result.
    emu.flags_mut().f_cf = src != 0;
    emu.flags_mut().f_of = false;
    emu.flags_mut().f_zf = result == 0;
    emu.flags_mut().f_sf = (result >> (sz - 1)) & 1 == 1;

    emu.set_operand_value(ins, 0, result);
    true
}
