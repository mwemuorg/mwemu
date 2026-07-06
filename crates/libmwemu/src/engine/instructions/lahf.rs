use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    //log::trace!("\tlahf: flags = {:?}", emu.flags);

    let mut result: u8 = 0;
    set_bit!(result, 0, emu.flag_cf() as u8);
    set_bit!(result, 1, true as u8);
    set_bit!(result, 2, emu.flag_pf() as u8);
    set_bit!(result, 3, false as u8);
    set_bit!(result, 4, emu.flag_af() as u8);
    set_bit!(result, 5, false as u8);
    set_bit!(result, 6, emu.flag_zf() as u8);
    set_bit!(result, 7, emu.flag_sf() as u8);
    emu.regs_mut().set_ah(result as u64);
    true
}
