use crate::emu::Emu;
use crate::{color, regs64};
use iced_x86::Instruction;

// VMOVD has two forms:
//   vmovd xmm, r/m32   -> xmm = zero_extend(r/m32)
//   vmovd r/m32, xmm   -> r/m32 = xmm[31:0]
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 2);

    let sz0 = emu.get_operand_sz(ins, 0);

    if sz0 == 32 {
        // vmovd r/m32, xmm : store the low 32 bits of the xmm source.
        let xmm = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
        emu.set_operand_value(ins, 0, (xmm as u32) as u64);
        return true;
    }

    // vmovd xmm/ymm, r/m32 : zero-extend the 32-bit source into the vector reg.
    let value = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v as u32 as u128,
        None => {
            log::trace!("error reading second operand");
            return false;
        }
    };

    match sz0 {
        128 => emu.set_operand_xmm_value_128(ins, 0, value),
        256 => emu.set_operand_ymm_value_256(ins, 0, regs64::U256::from(value as u64)),
        _ => return false,
    }
    true
}
