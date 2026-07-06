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
            let mem_addr = emu
                .get_operand_value(ins, 0, false)
                .expect("Fadd bad mem addr");

            let value = match mem_size {
                iced_x86::MemorySize::Float32 => emu.maps.read_f32(mem_addr).unwrap() as f64,
                iced_x86::MemorySize::Float64 => emu.maps.read_f64(mem_addr).unwrap(),
                _ => unreachable!(),
            };

            emu.fpu_mut().set_st(0, value);
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }
    emu.sync_fpu_ip();
    true
}
