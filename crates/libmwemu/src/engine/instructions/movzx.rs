use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    assert!(
        !(sz1 != 8 || sz0 != 16 && sz0 != 32)
            || (sz0 == 32 && sz1 == 16)
            || (sz0 == 64 && sz1 == 32)
            || (sz0 == 64 && sz1 == 16)
            || (sz0 == 64 && sz1 == 8)
    );

    let result: u64 = value1;

    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
