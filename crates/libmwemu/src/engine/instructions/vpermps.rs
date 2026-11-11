use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let (ilo, ihi) = avx::to_pair(try_operand!(emu.get_operand_ymm_value_256(ins, 1, true)));
    let (dlo, dhi) = avx::to_pair(try_operand!(emu.get_operand_ymm_value_256(ins, 2, true)));
    // Indices can select from either 128-bit half.
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
    emu.set_operand_ymm_value_256(ins, 0, avx::from_pair(nlo, nhi))
}
