use crate::arch::x86::regs::U256;
use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VCVTPD2DQ: f64 -> int32 (narrowing to xmm dest).
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let ssz = emu.get_operand_sz(ins, 1);
    let n = ssz / 64;
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
        let v = if i * 64 < 128 { slo } else { shi };
        let f = f64::from_bits(((v >> ((i * 64) % 128)) & 0xffff_ffff_ffff_ffff) as u64);
        let x = if false {
            f.trunc()
        } else {
            f.round_ties_even()
        };
        let d: i32 = if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
            i32::MIN
        } else {
            x as i32
        };
        r |= ((d as u32) as u128) << (i * 32);
    }
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}
