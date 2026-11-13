use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, OpKind, Register};

/// `out dx|imm8, al/ax/eax`: write an I/O port (see `in_`).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if emu.kernel.as_ref().is_none_or(|k| k.ledger_only) {
        log::warn!("out: port I/O outside kernel mode");
        return false;
    }
    let port = match ins.op0_kind() {
        OpKind::Immediate8 => ins.immediate8() as u16,
        _ => emu.regs().rdx as u16,
    };
    let (size, value) = match ins.op1_register() {
        Register::AL => (1, emu.regs().get_al()),
        Register::AX => (2, emu.regs().get_ax()),
        _ => (4, emu.regs().get_eax()),
    };
    emu.kernel_port_out(port, value, size);
    true
}
