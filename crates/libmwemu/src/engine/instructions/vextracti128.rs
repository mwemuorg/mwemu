use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VEXTRACTI128 dest(xmm/m128), src(ymm), imm8[0]: extract the selected 128-bit lane.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let (lo, hi) = avx::to_pair(
        emu.get_operand_ymm_value_256(ins, 1, true)
            .unwrap_or(crate::arch::x86::regs::U256::from(0)),
    );
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0);
    emu.set_operand_xmm_value_128(ins, 0, if imm & 1 != 0 { hi } else { lo });
    true
}
