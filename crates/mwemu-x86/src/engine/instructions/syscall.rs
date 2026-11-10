use crate::color;
use crate::emu::Emu;
use crate::layers;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if emu.cfg.trace_calls && !emu.os.is_linux() {
        log::trace!(
            "{} 0x{:x} SYSCALL nr=0x{:x}",
            emu.pos,
            ins.ip(),
            emu.regs().rax
        );
    }

    if emu.os.is_linux() {
        layers::linux().syscall64(emu);
    } else if emu.os.is_macos() {
        layers::macos().syscall_x86_64(emu);
    } else {
        layers::windows().syscall64(emu);
    }
    true
}
