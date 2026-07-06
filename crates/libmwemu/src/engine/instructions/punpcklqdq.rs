use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    if sz0 == 128 && sz1 == 128 {
        let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value0");
                return false;
            }
        };

        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
        // PUNPCKLQDQ: DEST[63:0] = DEST[63:0]; DEST[127:64] = SRC[63:0]
        // (operates on full 64-bit quadwords, not 32-bit words).
        let value0_low_qword = value0 as u64;
        let value1_low_qword = value1 as u64;
        let result = ((value1_low_qword as u128) << 64) | (value0_low_qword as u128);

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else {
        log::trace!("unimplemented case punpcklqdq {} {}", sz0, sz1);
        return false;
    }
    true
}
