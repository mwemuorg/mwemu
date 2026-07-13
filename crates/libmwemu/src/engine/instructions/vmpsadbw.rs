use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VMPSADBW: VEX op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let so = (((imm >> 2) & 1) as u32) * 4;
        let dof = ((imm & 3) as u32) * 4;
        let byte = |v: u128, i: u32| -> i32 {
            if i < 16 {
                ((v >> (i * 8)) & 0xff) as i32
            } else {
                0
            }
        };
        let mut r = 0u128;
        for j in 0..8u32 {
            let mut sum = 0u32;
            for k in 0..4u32 {
                sum += (byte(a, dof + j + k) - byte(b, so + k)).unsigned_abs();
            }
            r |= (sum as u128) << (j * 16);
        }
        r
    })
}
