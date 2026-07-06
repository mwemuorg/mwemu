use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() <= 1);

    let mut low: u64 = emu.regs().get_al();
    let high: u64 = emu.regs().get_ah();

    let imm: u64 = if ins.op_count() == 0 {
        10
    } else {
        match emu.get_operand_value(ins, 0, true) {
            Some(v) => v,
            None => return false,
        }
    };

    low = (low + (imm * high)) & 0xff;
    emu.regs_mut().set_al(low);
    emu.regs_mut().set_ah(0);

    emu.flags_overwrite_mut().calc_flags(low, 8);
    true
}
