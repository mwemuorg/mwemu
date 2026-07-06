use crate::color;
use crate::emu::Emu;
use crate::engine::logic;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 1);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let pre_rax = emu.regs().rax;
    let pre_rdx = emu.regs().rdx;

    let sz = emu.get_operand_sz(ins, 0);
    match sz {
        64 => logic::mul64(emu, value0),
        32 => logic::mul32(emu, value0),
        16 => logic::mul16(emu, value0),
        8 => logic::mul8(emu, value0),
        _ => unimplemented!("wrong size"),
    }
    true
}
