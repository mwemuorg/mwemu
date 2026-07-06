use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Yellow"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if ins.op_count() != 1 {
        unimplemented!("weird variant of jmp");
    }

    let addr = match emu.get_jump_value(ins, 0) {
        Some(a) => a,
        None => return false,
    };

    if emu.cfg.is_x64() {
        emu.set_rip(addr, false)
    } else {
        emu.set_eip(addr, false)
    }
    //true
}
