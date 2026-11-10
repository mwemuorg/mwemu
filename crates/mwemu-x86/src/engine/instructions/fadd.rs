use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if ins.op_count() == 2 {
        let i = emu.fpu().reg_to_id(ins.op_register(0));
        let j = emu.fpu().reg_to_id(ins.op_register(1));
        emu.fpu_mut().add(i, j);
    } else if ins.op_count() == 1 {
        if ins.op_kind(0) == iced_x86::OpKind::Memory {
            let mem_size = ins.memory_size();
            let mem_addr = match emu.get_operand_value(ins, 0, false) {
                Some(v) => v,
                None => {
                    log::trace!("fadd: bad memory address");
                    return false;
                }
            };

            let value = match mem_size {
                iced_x86::MemorySize::Float32 => match emu.maps.read_f32(mem_addr) {
                    Some(v) => v as f64,
                    None => {
                        log::trace!("fadd: cannot read f32 at 0x{:x}", mem_addr);
                        return false;
                    }
                },
                iced_x86::MemorySize::Float64 => match emu.maps.read_f64(mem_addr) {
                    Some(v) => v,
                    None => {
                        log::trace!("fadd: cannot read f64 at 0x{:x}", mem_addr);
                        return false;
                    }
                },
                _ => {
                    log::warn!("fadd: unsupported memory size {:?}", mem_size);
                    return false;
                }
            };

            emu.fpu_mut().set_st(0, value);
        } else {
            log::warn!("fadd: unexpected operand kind");
            return false;
        }
    } else {
        log::warn!("fadd: unexpected operand count {}", ins.op_count());
        return false;
    }
    emu.sync_fpu_ip();
    true
}
