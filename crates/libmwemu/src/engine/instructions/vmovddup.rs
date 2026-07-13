use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VMOVDDUP: VEX op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::unop(emu, ins, |a| {
        let lo = a & 0xffff_ffff_ffff_ffff;
        lo | (lo << 64)
    })
}
