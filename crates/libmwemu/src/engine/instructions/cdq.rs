use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let num: i64 = emu.regs().get_eax() as u32 as i32 as i64; // sign-extend
    let unum: u64 = num as u64;
    emu.regs_mut().set_edx((unum & 0xffffffff00000000) >> 32);
    // preserve upper 64-bits from getting overriden
    let rax_upper = emu.regs().rax >> 32;
    emu.regs_mut().rax = (rax_upper << 32) | (unum & 0xffffffff);
    true
}
