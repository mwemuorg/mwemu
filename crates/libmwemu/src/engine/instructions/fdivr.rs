use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if ins.op_count() == 2 {
        let st0 = emu.fpu_mut().get_st(0);
        let value1 = emu.get_operand_value(ins, 1, false).unwrap_or(0);
        let stn = emu.fpu_mut().get_st(value1 as usize);
        emu.fpu_mut().set_st(0, stn / st0);
    } else if ins.op_count() == 1 && ins.op_kind(0) == iced_x86::OpKind::Memory {
        let mem_addr = match emu.get_operand_value(ins, 0, false) {
            Some(v) => v,
            None => return false,
        };
        let mem_val = match ins.memory_size() {
            iced_x86::MemorySize::Float32 => match emu.maps.read_f32(mem_addr) {
                Some(v) => v as f64,
                None => return false,
            },
            iced_x86::MemorySize::Float64 => match emu.maps.read_f64(mem_addr) {
                Some(v) => v,
                None => return false,
            },
            iced_x86::MemorySize::Int16 => {
                let v = emu.maps.read_word(mem_addr).unwrap_or(0) as i16;
                v as f64
            }
            iced_x86::MemorySize::Int32 => {
                let v = emu.maps.read_dword(mem_addr).unwrap_or(0) as i32;
                v as f64
            }
            _ => {
                log::warn!("fdivr: unsupported memory size {:?}", ins.memory_size());
                return false;
            }
        };
        let st0 = emu.fpu_mut().get_st(0);
        emu.fpu_mut().set_st(0, mem_val / st0);
    } else {
        log::warn!("fdivr: unexpected operand count {}", ins.op_count());
        return false;
    }

    emu.sync_fpu_ip();
    true
}
