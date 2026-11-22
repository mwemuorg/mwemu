use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::exception::types::ExceptionType;
use crate::fpu::f80::F80;
use iced_x86::{Instruction, MemorySize, OpKind};

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let mut value = F80::new();
    if ins.op_kind(0) == OpKind::Register {
        let index = emu.fpu().reg_to_id(ins.op_register(0));
        value = emu.fpu_mut().get_st_f80_copy(index);
    } else {
        match ins.memory_size() {
            MemorySize::Float32 => {
                let bits = try_operand!(emu.get_operand_value(ins, 0, true)) as u32;
                value.set_f64(f64::from(f32::from_bits(bits)));
            }
            MemorySize::Float64 => {
                let bits = try_operand!(emu.get_operand_value(ins, 0, true));
                value.set_f64(f64::from_bits(bits));
            }
            MemorySize::Float80 => {
                let addr = try_operand!(emu.get_operand_value(ins, 0, false));
                let bytes = try_operand!(emu.read_instruction_bytes::<10>(
                    addr,
                    10,
                    ExceptionType::BadAddressDereferencing
                ));
                value.set_bytes(&bytes);
            }
            _ => return false,
        }
    }
    if emu.fpu().st.get_depth() >= 8 {
        emu.fpu_mut().set_status_c1(true);
    } else {
        emu.fpu_mut().set_status_c1(false);
        emu.fpu_mut().push_f80(value);
    }
    emu.sync_fpu_ip();
    true
}
