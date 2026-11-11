use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let s1 = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let v = (try_operand!(emu.get_operand_value(ins, 2, true)) as u128) & 0xff;
    let idx = (try_operand!(emu.get_operand_value(ins, 3, true)) as u32) & 0xf;
    let sh = idx * 8;
    emu.set_operand_xmm_value_128(ins, 0, (s1 & !(0xff_u128 << sh)) | (v << sh))
}
