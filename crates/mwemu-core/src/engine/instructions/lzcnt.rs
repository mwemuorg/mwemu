use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    // Count leading zeros at the operand width — a 16-bit LZCNT of 0 is 16, not
    // the 64 that u64::leading_zeros would report.
    let sz = emu.get_operand_sz(ins, 0);
    let lz = match sz {
        16 => (src as u16).leading_zeros() as u64,
        32 => (src as u32).leading_zeros() as u64,
        _ => src.leading_zeros() as u64,
    };

    if !emu.set_operand_value(ins, 0, lz) {
        return false;
    }

    // CF set when the source is zero (result equals the operand width),
    // ZF set when the result is zero; the rest are cleared.
    let f = emu.flags_mut();
    f.f_cf = src == 0;
    f.f_zf = lz == 0;
    f.f_of = false;
    f.f_sf = false;
    f.f_af = false;
    f.f_pf = false;
    true
}
