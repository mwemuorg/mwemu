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

    if mem_val == edx_eax {
        emu.flags_mut().f_zf = true;
        let ecx = emu.regs().rcx & 0xffffffff;
        let ebx = emu.regs().rbx & 0xffffffff;
        let ecx_ebx = (ecx << 32) | ebx;

        if !emu.set_operand_value(ins, 0, ecx_ebx) {
            return false;
        }
    } else {
        emu.flags_mut().f_zf = false;
        emu.regs_mut().rax = mem_val & 0xffffffff;
        emu.regs_mut().rdx = (mem_val >> 32) & 0xffffffff;
    }
    true
}
