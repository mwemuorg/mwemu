use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() <= 1);

    let imm = if ins.op_count() == 0 {
        10
    } else {
        match emu.get_operand_value(ins, 0, true) {
            Some(v) => v & 0xff,
            None => return false,
        }
    };

    if imm == 0 {
        return false;
    }

    let al = emu.regs().get_al() & 0xff;
    let ah = (al / imm) & 0xff;
    let new_al = (al % imm) & 0xff;
    emu.regs_mut().set_ah(ah);
    emu.regs_mut().set_al(new_al);

    emu.flags_overwrite_mut().calc_flags(new_al, 8);
    true
}
