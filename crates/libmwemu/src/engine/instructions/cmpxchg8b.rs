use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Orange"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let mem_val = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let edx = emu.regs().rdx & 0xffffffff;
    let eax = emu.regs().rax & 0xffffffff;
    let edx_eax = (edx << 32) | eax;

    let equal = mem_val == edx_eax;
    let value = if equal {
        (emu.regs().get_ecx() << u32::BITS) | emu.regs().get_ebx()
    } else {
        mem_val
    };
    if !emu.set_operand_value(ins, 0, value) {
        return false;
    }
    emu.flags_mut().f_zf = equal;
    if !equal {
        emu.regs_mut().set_eax(mem_val as u32 as u64);
        emu.regs_mut().set_edx(mem_val >> u32::BITS);
    }
    true
}
