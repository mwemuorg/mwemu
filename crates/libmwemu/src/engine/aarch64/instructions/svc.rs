use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let imm = match ins.operands[0] {
        Operand::Immediate(v) => v as u64,
        Operand::Imm16(v) => v as u64,
        _ => 0,
    };

    if emu.os.is_macos() {
        // macOS aarch64: x16 = syscall number, SVC #0x80
        log::trace!(
            "SVC #0x{:x} at 0x{:x} (x16=0x{:x} x0=0x{:x})",
            imm,
            emu.regs_aarch64().pc,
            emu.regs_aarch64().x[16],
            emu.regs_aarch64().x[0],
        );
        crate::syscall::macos::syscall_aarch64::gateway(emu);
    } else if emu.os.is_linux() {
        // Linux aarch64: x8 = syscall number, SVC #0
        log::trace!(
            "SVC #0x{:x} at 0x{:x} (x8=0x{:x} x0=0x{:x})",
            imm,
            emu.regs_aarch64().pc,
            emu.regs_aarch64().x[8],
            emu.regs_aarch64().x[0],
        );
        crate::syscall::linux::syscall_aarch64::gateway(emu);
    } else {
        log::warn!(
            "SVC #{} at 0x{:x} but os is not macOS or Linux",
            imm,
            emu.regs_aarch64().pc,
        );
    }
    true
}
