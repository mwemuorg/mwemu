use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("LightCyan"), &DecodedInstruction::X86(*ins));
    let (zf, cf) = if emu.get_operand_sz(ins, 0) == 256 {
        let a = try_operand!(emu.get_operand_ymm_value_256(ins, 0, true));
        let b = try_operand!(emu.get_operand_ymm_value_256(ins, 1, true));
        ((a & b).is_zero(), (b & !a).is_zero())
    } else {
        let a = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
        let b = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
        ((a & b) == 0, (b & !a) == 0)
    };
    let f = emu.flags_mut();
    f.f_zf = zf;
    f.f_cf = cf;
    f.f_of = false;
    f.f_sf = false;
    f.f_af = false;
    f.f_pf = false;
    true
}
