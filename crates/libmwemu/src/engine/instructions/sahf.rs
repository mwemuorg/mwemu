use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// SAHF: load SF, ZF, AF, PF and CF from AH.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let ah = emu.regs().get_ah() as u8;
    emu.flags_mut().f_cf = ah & 0x01 != 0;
    emu.flags_mut().f_pf = ah & 0x04 != 0;
    emu.flags_mut().f_af = ah & 0x10 != 0;
    emu.flags_mut().f_zf = ah & 0x40 != 0;
    emu.flags_mut().f_sf = ah & 0x80 != 0;
    true
}
