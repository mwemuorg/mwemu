use crate::arch::x86::regs::U256;
use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VPERMPS dest, indices, data: gather 8 dwords across the full 256 bits.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let (ilo, ihi) = avx::to_pair(
        emu.get_operand_ymm_value_256(ins, 1, true)
            .unwrap_or(U256::from(0)),
    );
    let (dlo, dhi) = avx::to_pair(
        emu.get_operand_ymm_value_256(ins, 2, true)
            .unwrap_or(U256::from(0)),
    );
    let dw = |n: u32| {
        if n < 4 {
            (dlo >> (n * 32)) & 0xffffffff
        } else {
            (dhi >> ((n - 4) * 32)) & 0xffffffff
        }
    };
    let idx = |n: u32| {
        let v = if n < 4 {
            ilo >> (n * 32)
        } else {
            ihi >> ((n - 4) * 32)
        };
        (v & 7) as u32
    };
    let mut nlo = 0u128;
    let mut nhi = 0u128;
    for i in 0..8u32 {
        let val = dw(idx(i));
        if i < 4 {
            nlo |= val << (i * 32)
        } else {
            nhi |= val << ((i - 4) * 32)
        }
    }
    emu.set_operand_ymm_value_256(ins, 0, avx::from_pair(nlo, nhi));
    true
}
