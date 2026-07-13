use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VCVTPH2PS: packed f16 -> f32 (dest width sets lane count).
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dsz = emu.get_operand_sz(ins, 0);
    let n = dsz / 32;
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut lo = 0u128;
    let mut hi = 0u128;
    for i in 0..n {
        let h = ((src >> (i * 16)) & 0xffff) as u16;
        let f = avx::f16_to_f32(h) as u128;
        let pos = i * 32;
        if pos < 128 {
            lo |= f << pos
        } else {
            hi |= f << (pos - 128)
        }
    }
    if dsz == 128 {
        emu.set_operand_xmm_value_128(ins, 0, lo)
    } else {
        emu.set_operand_ymm_value_256(ins, 0, avx::from_pair(lo, hi))
    }
    true
}
