use crate::arch::x86::regs::U256;
use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VCVTPS2PH dest, src, imm8: packed f32 -> f16 (round-to-nearest-even).
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let ssz = emu.get_operand_sz(ins, 1);
    let n = ssz / 32;
    let (slo, shi) = if ssz == 256 {
        avx::to_pair(
            emu.get_operand_ymm_value_256(ins, 1, true)
                .unwrap_or(U256::from(0)),
        )
    } else {
        (emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0), 0)
    };
    let mut r = 0u128;
    for i in 0..n {
        let v = if i * 32 < 128 { slo } else { shi };
        let f = ((v >> ((i * 32) % 128)) & 0xffffffff) as u32;
        r |= (avx::f32_to_f16(f) as u128) << (i * 16);
    }
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}
