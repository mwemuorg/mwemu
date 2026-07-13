use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PTEST dest, src: ZF = ((dest AND src) == 0); CF = ((src AND NOT dest) == 0).
// OF, SF, AF and PF are cleared.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    emu.flags_mut().f_zf = (dest & src) == 0;
    emu.flags_mut().f_cf = (src & !dest) == 0;
    emu.flags_mut().f_of = false;
    emu.flags_mut().f_sf = false;
    emu.flags_mut().f_af = false;
    emu.flags_mut().f_pf = false;
    true
}
