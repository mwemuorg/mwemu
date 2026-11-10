use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if emu.fpu().st.get_depth() >= 8 {
        emu.fpu_mut().set_status_c1(true);
    } else {
        emu.fpu_mut().set_status_c1(false);
        emu.fpu_mut().st.dec_top();

        let value1 = match emu.get_operand_value(ins, 0, true) {
            Some(v) => v as i64 as f64,
            None => return false,
        };

        emu.fpu_mut().st.push_f64(value1);
        emu.sync_fpu_ip();
    }
    true
}
