use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// `VMOVUPS` — VEX-encoded move of unaligned packed single-precision floats.
/// Bit-for-bit this is the same data movement as `VMOVDQU`/`VMOVUPD` (the
/// "type" only matters to forwarding/decoder, not to the bytes moved), so we
/// handle both the 128-bit (xmm) and 256-bit (ymm) forms identically.
pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);
    let sz_max = sz0.max(sz1);

    match sz_max {
        128 => {
            let source = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("vmovups: error reading xmm/m128 source operand");
                    return false;
                }
            };
            emu.set_operand_xmm_value_128(ins, 0, source);
        }
        256 => {
            let source = match emu.get_operand_ymm_value_256(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("vmovups: error reading ymm/m256 source operand");
                    return false;
                }
            };
            emu.set_operand_ymm_value_256(ins, 0, source);
        }
        _ => {
            unimplemented!("vmovups: unsupported operand size {}", sz_max);
        }
    }
    true
}
