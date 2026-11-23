use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::fpu::f80::F80;
use iced_x86::{Instruction, MemorySize};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let (destination, divisor) = if ins.op_count() == 2 {
        let destination = emu.fpu().reg_to_id(ins.op_register(0));
        let source = emu.fpu().reg_to_id(ins.op_register(1));
        (destination, emu.fpu_mut().get_st_f80_copy(source))
    } else {
        let bits = try_operand!(emu.get_operand_value(ins, 0, true));
        let operand = match ins.memory_size() {
            MemorySize::Float32 => f64::from(f32::from_bits(bits as u32)),
            MemorySize::Float64 => f64::from_bits(bits),
            _ => return false,
        };
        let mut divisor = F80::new();
        divisor.set_f64(operand);
        (0, divisor)
    };
    let mut value = emu.fpu_mut().get_st_f80_copy(destination);
    value.divide(divisor, emu.fpu().get_ctrl());
    emu.fpu_mut().set_st_u80(destination, value.get());
    emu.sync_fpu_ip();
    true
}
