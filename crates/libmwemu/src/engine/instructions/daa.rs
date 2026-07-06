use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let old_al = emu.regs().get_al();
    let old_cf = emu.flag_cf();
    emu.flags_mut().f_cf = false;

    if (emu.regs().get_al() & 0x0f > 9) || emu.flag_af() {
        let sum = emu.regs().get_al() + 6;
        emu.regs_mut().set_al(sum & 0xff);
        if sum > 0xff {
            emu.flags_mut().f_cf = true;
        } else {
            emu.flags_mut().f_cf = old_cf;
        }

        emu.flags_mut().f_af = true;
    } else {
        emu.flags_mut().f_af = false;
    }

    if old_al > 0x99 || old_cf {
        let al = emu.regs().get_al() + 0x60;
        emu.regs_mut().set_al(al);
        emu.flags_mut().f_cf = true;
    } else {
        emu.flags_mut().f_cf = false;
    }
    true
}
