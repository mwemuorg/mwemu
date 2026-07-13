use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VMOVMSKPD: gather lane sign bits (128 or 256) into a GPR.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let sz = emu.get_operand_sz(ins, 1);
    let (lo, hi) = if sz == 256 {
        avx::to_pair(
            emu.get_operand_ymm_value_256(ins, 1, true)
                .unwrap_or(crate::arch::x86::regs::U256::from(0)),
        )
    } else {
        (emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0), 0)
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
    if !emu.set_operand_value(ins, 0, mask) {
        return false;
    }
    true
}
