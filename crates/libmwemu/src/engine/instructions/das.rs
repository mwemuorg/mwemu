use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// DAS: decimal adjust AL after a packed-BCD subtraction (SDM pseudo-code).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let old_al = emu.regs().get_al();
    let old_cf = emu.flag_cf();
    let mut al = old_al;
    let mut cf = false;
    let mut af = false;
    if (old_al & 0x0f) > 9 || emu.flag_af() {
        cf = old_cf || old_al < 6;
        al = al.wrapping_sub(6) & 0xff;
        af = true;
    }
    if old_al > 0x99 || old_cf {
        al = al.wrapping_sub(0x60) & 0xff;
        cf = true;
    }
    emu.regs_mut().set_al(al);
    let flags = emu.flags_mut();
    flags.f_cf = cf;
    flags.f_af = af;
    flags.calc_flags(al, 8);
    true
}
