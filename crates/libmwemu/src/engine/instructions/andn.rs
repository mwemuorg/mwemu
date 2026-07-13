use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// ANDN dest, src1, src2: dest = (~src1) & src2
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let src1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let src2 = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    let mask: u64 = if sz >= 64 { u64::MAX } else { (1u64 << sz) - 1 };
    let result = (!src1) & src2 & mask;

    // SF and ZF reflect the result; CF and OF are cleared (AF/PF undefined).
    emu.flags_mut().f_cf = false;
    emu.flags_mut().f_of = false;
    emu.flags_mut().f_zf = result == 0;
    emu.flags_mut().f_sf = (result >> (sz - 1)) & 1 == 1;

    emu.set_operand_value(ins, 0, result);
    true
}
