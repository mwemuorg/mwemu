use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let elapsed = emu.now.elapsed();
    let cycles: u64 = elapsed.as_nanos() as u64;
    emu.regs_mut().rax = cycles & 0xffffffff;
    emu.regs_mut().rdx = cycles >> 32;

    if emu.cfg.verbose >= 1 {
        log::trace!(
            "\t{}:0x{:x} rdtsc value: {} {}",
            emu.pos,
            emu.regs().rip,
            cycles & 0xffffffff,
            cycles >> 32
        );
    }

    true
}
