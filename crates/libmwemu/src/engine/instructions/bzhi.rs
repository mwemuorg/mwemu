use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// BZHI dest, src, index : dest = src with bits >= index[7:0] zeroed. CF if index>size-1.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let sz = emu.get_operand_sz(ins, 0) as u64;
    let src = emu.get_operand_value(ins, 1, true).unwrap_or(0);
    let idx = emu.get_operand_value(ins, 2, true).unwrap_or(0) & 0xff;

    let result = if idx >= sz {
        src
    } else {
        src & ((1u64 << idx) - 1)
    };
    let result = if sz == 32 {
        result & 0xffffffff
    } else {
        result
    };

    emu.flags_mut().f_cf = idx > sz - 1;
    emu.flags_mut().f_of = false;
    emu.flags_mut().f_zf = result == 0;
    emu.flags_mut().f_sf = (result >> (sz - 1)) & 1 == 1;
    emu.set_operand_value(ins, 0, result);
    true
}
