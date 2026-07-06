use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// `VXORPS` — VEX-encoded bitwise XOR of packed single-precision floats:
/// `dest = src1 ^ src2`. Bit-for-bit identical to `VPXOR`/`VXORPD` (XOR is
/// type-agnostic), handled for both the 128-bit (xmm) and 256-bit (ymm) forms.
pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    match emu.get_operand_sz(ins, 0) {
        128 => {
            let source1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("vxorps: error reading xmm src1 operand");
                    return false;
                }
            };
            let source2 = match emu.get_operand_xmm_value_128(ins, 2, true) {
                Some(v) => v,
                None => {
                    log::trace!("vxorps: error reading xmm/m128 src2 operand");
                    return false;
                }
            };
            emu.set_operand_xmm_value_128(ins, 0, source1 ^ source2);
        }
        256 => {
            let source1 = match emu.get_operand_ymm_value_256(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("vxorps: error reading ymm src1 operand");
                    return false;
                }
            };
            let source2 = match emu.get_operand_ymm_value_256(ins, 2, true) {
                Some(v) => v,
                None => {
                    log::trace!("vxorps: error reading ymm/m256 src2 operand");
                    return false;
                }
            };
            emu.set_operand_ymm_value_256(ins, 0, source1 ^ source2);
        }
        _ => unreachable!(""),
    }
    true
}
