use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let ssz = emu.get_operand_sz(ins, 1);
    let n = ssz / 32;
    let (slo, shi) = if ssz == 256 {
        avx::to_pair(try_operand!(emu.get_operand_ymm_value_256(ins, 1, true)))
    } else {
        (try_operand!(emu.get_operand_xmm_value_128(ins, 1, true)), 0)
    };
    let mut r = 0u128;
    for i in 0..n {
        let v = if i * 32 < 128 { slo } else { shi };
        let f = ((v >> ((i * 32) % 128)) & 0xffffffff) as u32;
        r |= (avx::f32_to_f16(f) as u128) << (i * 16);
    }
    emu.set_operand_xmm_value_128(ins, 0, r)
}
