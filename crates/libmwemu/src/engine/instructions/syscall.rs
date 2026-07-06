use crate::color;
use crate::emu::Emu;
use crate::syscall::linux;
use crate::syscall::macos;
use crate::syscall::windows;
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
        linux::syscall64::gateway(emu);
    } else if emu.os.is_macos() {
        macos::syscall_x86_64::gateway(emu);
    } else {
        windows::syscall64::gateway(emu);
    }
    true
}
