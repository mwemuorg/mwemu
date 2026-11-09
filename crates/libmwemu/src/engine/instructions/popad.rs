use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::{Instruction, Register};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));
    const REGISTERS: [Register; 8] = [
        Register::EDI,
        Register::ESI,
        Register::EBP,
        Register::ESP,
        Register::EBX,
        Register::EDX,
        Register::ECX,
        Register::EAX,
    ];
    let original_sp = emu.sp();
    let mut values = [0; REGISTERS.len()];
    for (register, value) in REGISTERS.into_iter().zip(&mut values) {
        if register == Register::ESP {
            // The saved ESP slot is skipped without reading it.
            let sp = (emu.sp() as u32).wrapping_add(size_of::<u32>() as u32);
            emu.set_sp(sp as u64);
            continue;
        }
        let Some(popped) = emu.stack_pop32(false) else {
            emu.set_sp(original_sp);
            return false;
        };
        *value = popped as u64;
    }
    for (register, value) in REGISTERS.into_iter().zip(values) {
        if register != Register::ESP {
            emu.regs_mut().set_reg(register, value);
        }
    }
    true
}
