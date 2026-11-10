use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Orange"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let mem_val = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let rdx = emu.regs().rdx;
    let rax = emu.regs().rax;
    let rdx_rax = ((rdx as u128) << 64) | (rax as u128);

    let equal = mem_val == rdx_rax;
    let value = if equal {
        ((emu.regs().rcx as u128) << u64::BITS) | emu.regs().rbx as u128
    } else {
        mem_val
    };
    // CMPXCHG performs a write cycle even when the comparison fails.
    if !emu.set_operand_xmm_value_128(ins, 0, value) {
        return false;
    }
    emu.flags_mut().f_zf = equal;
    if !equal {
        emu.regs_mut().rax = mem_val as u64;
        emu.regs_mut().rdx = (mem_val >> u64::BITS) as u64;
    }
    true
}
