use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if emu.cfg.is_x64() {
        emu.regs_mut().rsp = emu.regs().rbp;
        emu.regs_mut().rbp = match emu.stack_pop64(true) {
            Some(v) => v,
            None => return false,
        };
    } else {
        let esp = emu.regs().get_ebp();
        emu.regs_mut().set_esp(esp);
        let val = match emu.stack_pop32(true) {
            Some(v) => v as u64,
            None => return false,
        };
        emu.regs_mut().set_ebp(val);
    }
    true
}
