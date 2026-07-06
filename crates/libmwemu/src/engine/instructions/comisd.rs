use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// COMISD : compare the low scalar doubles and set ZF/PF/CF (unordered => all set).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 2);

    let value1 = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let value2 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let f1 = f64::from_bits((value1 & 0xFFFFFFFFFFFFFFFF) as u64);
    let f2 = f64::from_bits((value2 & 0xFFFFFFFFFFFFFFFF) as u64);

    if f1.is_nan() || f2.is_nan() {
        emu.flags_mut().f_zf = true;
        emu.flags_mut().f_pf = true;
        emu.flags_mut().f_cf = true;
    } else {
        emu.flags_mut().f_pf = false;
        emu.flags_mut().f_zf = f1 == f2;
        emu.flags_mut().f_cf = f1 < f2;
    }
    true
}
