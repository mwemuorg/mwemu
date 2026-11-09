use crate::emu::Emu;
use crate::{color, regs64};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let mask_lower = regs64::U256::from(0xffffffffffffffffu64);
    let mask = mask_lower | (mask_lower << 64);

    emu.regs_mut().ymm0 &= mask;
    emu.regs_mut().ymm1 &= mask;
    emu.regs_mut().ymm2 &= mask;
    emu.regs_mut().ymm3 &= mask;
    emu.regs_mut().ymm4 &= mask;
    emu.regs_mut().ymm5 &= mask;
    emu.regs_mut().ymm6 &= mask;
    emu.regs_mut().ymm7 &= mask;
    emu.regs_mut().ymm8 &= mask;
    emu.regs_mut().ymm9 &= mask;
    emu.regs_mut().ymm10 &= mask;
    emu.regs_mut().ymm11 &= mask;
    emu.regs_mut().ymm12 &= mask;
    emu.regs_mut().ymm13 &= mask;
    emu.regs_mut().ymm14 &= mask;
    emu.regs_mut().ymm15 &= mask;
    true
}
