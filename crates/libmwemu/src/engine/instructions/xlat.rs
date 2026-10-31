use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, Register};

/// XLAT/XLATB: AL = [table + zero-extended AL], the table base being BX, EBX
/// or RBX depending on the address size.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let table = match ins.memory_base() {
        Register::BX => emu.regs().get_bx(),
        Register::EBX => emu.regs().get_ebx(),
        _ => emu.regs().rbx,
    };
    let addr = table.wrapping_add(emu.regs().get_al());
    match emu.maps.read_byte(addr) {
        Some(v) => {
            emu.regs_mut().set_al(v as u64);
            true
        }
        None => {
            log::trace!("xlat: cannot read table entry at 0x{:x}", addr);
            false
        }
    }
}
