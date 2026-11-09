use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if ins.op_count() != 0 {
        return false;
    }

    let al = emu.regs().get_al();
    let ah = emu.regs().get_ah();
    let af = emu.flag_af();

    if (al & 0x0f) > 9 || af {
        let new_al = (al.wrapping_sub(6)) & 0x0f;
        let new_ah = ah.wrapping_sub(1) & 0xff;
        emu.regs_mut().set_al(new_al);
        emu.regs_mut().set_ah(new_ah);
        emu.flags_mut().f_af = true;
        emu.flags_mut().f_cf = true;
    } else {
        emu.regs_mut().set_al(al & 0x0f);
        emu.flags_mut().f_af = false;
        emu.flags_mut().f_cf = false;
    }

    true
}
