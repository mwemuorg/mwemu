use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::fpu::f80::F80;
use iced_x86::{Instruction, MemorySize};

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let raw = try_operand!(emu.get_operand_value(ins, 0, true));
    let signed = match ins.memory_size() {
        MemorySize::Int16 => i64::from(raw as i16),
        MemorySize::Int32 => i64::from(raw as i32),
        MemorySize::Int64 => raw as i64,
        _ => return false,
    };
    if emu.fpu().st.get_depth() >= 8 {
        emu.fpu_mut().set_status_c1(true);
    } else {
        let value = F80 {
            st: F80::encode_from_u128(u128::from(signed.unsigned_abs()), signed < 0),
        };
        emu.fpu_mut().set_status_c1(false);
        emu.fpu_mut().push_f80(value);
    }
    emu.sync_fpu_ip();
    true
}
