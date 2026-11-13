use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, OpKind, Register};

/// `in al/ax/eax, dx|imm8`: read an I/O port.
///
/// Ports are modelled as a register file in kernel mode (a read returns the
/// last value written, or a preset), and the access is logged as a
/// privileged operation: port I/O from an IOCTL path is a BYOVD primitive.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if emu.kernel.as_ref().is_none_or(|k| k.ledger_only) {
        log::warn!("in: port I/O outside kernel mode");
        return false;
    }
    let port = match ins.op1_kind() {
        OpKind::Immediate8 => ins.immediate8() as u16,
        _ => emu.regs().rdx as u16,
    };
    let size = match ins.op0_register() {
        Register::AL => 1,
        Register::AX => 2,
        _ => 4,
    };
    let v = emu.kernel_port_in(port, size);
    match size {
        1 => emu.regs_mut().set_al(v),
        2 => emu.regs_mut().set_ax(v),
        _ => emu.regs_mut().set_eax(v),
    }
    true
}
