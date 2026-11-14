use crate::color;
use crate::emu::Emu;
use iced_x86::{Instruction, Mnemonic};

/// `cli` / `sti`: interrupt flag. Interrupts are not modelled, so the flag
/// is simply tracked; a driver toggling it around an MSR or port access is
/// the usual shape, and the surrounding access is what gets logged.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if emu.kernel.as_ref().is_none_or(|k| k.ledger_only) {
        log::warn!("{:?} outside kernel mode", ins.mnemonic());
        return false;
    }
    emu.flags_mut().f_if = ins.mnemonic() == Mnemonic::Sti;
    true
}
