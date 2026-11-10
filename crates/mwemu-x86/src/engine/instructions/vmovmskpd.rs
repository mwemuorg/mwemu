use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let sz = emu.get_operand_sz(ins, 1);
    let (lo, hi) = if sz == 256 {
        avx::to_pair(try_operand!(emu.get_operand_ymm_value_256(ins, 1, true)))
    } else {
        (try_operand!(emu.get_operand_xmm_value_128(ins, 1, true)), 0)
    };
    let n = sz / 64;
    let mut mask = 0u64;
    for i in 0..n {
        let v = if i * 64 < 128 { lo } else { hi };
        let sh = (i * 64) % 128 + 64 - 1;
        if (v >> sh) & 1 == 1 {
            mask |= 1 << i;
        }
    }
    emu.set_operand_value(ins, 0, mask)
}
