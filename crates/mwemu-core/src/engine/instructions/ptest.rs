use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("LightCyan"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    emu.flags_mut().f_zf = (dest & src) == 0;
    emu.flags_mut().f_cf = (src & !dest) == 0;
    emu.flags_mut().f_of = false;
    emu.flags_mut().f_sf = false;
    emu.flags_mut().f_af = false;
    emu.flags_mut().f_pf = false;
    true
}
