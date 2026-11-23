use std::cmp::Ordering;

use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::fpu::{FPU_INVALID_OPERATION, f80::F80};
use iced_x86::{Instruction, MemorySize, OpKind};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let st0 = emu.fpu_mut().get_st_f80_copy(0);
    let other = if ins.op_count() == 2 {
        let index = emu.fpu().reg_to_id(ins.op_register(1));
        emu.fpu_mut().get_st_f80_copy(index)
    } else if ins.op_count() == 1 && ins.op_kind(0) == OpKind::Memory {
        let bits = try_operand!(emu.get_operand_value(ins, 0, true));
        let value = match ins.memory_size() {
            MemorySize::Float32 => f64::from(f32::from_bits(bits as u32)),
            MemorySize::Float64 => f64::from_bits(bits),
            _ => return false,
        };
        let mut other = F80::new();
        other.set_f64(value);
        other
    } else if ins.op_count() == 0 {
        emu.fpu_mut().get_st_f80_copy(1)
    } else {
        return false;
    };

    let relation = st0.numeric_cmp(other);
    let fpu = emu.fpu_mut();
    fpu.set_status_c0(matches!(relation, None | Some(Ordering::Less)));
    fpu.set_status_c1(false);
    fpu.set_status_c2(relation.is_none());
    fpu.set_status_c3(matches!(relation, None | Some(Ordering::Equal)));
    if relation.is_none() {
        fpu.status |= FPU_INVALID_OPERATION;
    }
    emu.sync_fpu_ip();
    true
}
