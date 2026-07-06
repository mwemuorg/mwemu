use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Orange"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(v) => v,
        None => return false,
    };

    let mem_val = match emu.maps.read_128bits_le(addr) {
        Some(v) => v,
        None => return false,
    };

    let rdx = emu.regs().rdx;
    let rax = emu.regs().rax;
    let rdx_rax = ((rdx as u128) << 64) | (rax as u128);

    if mem_val == rdx_rax {
        emu.flags_mut().f_zf = true;
        let rcx = emu.regs().rcx;
        let rbx = emu.regs().rbx;
        let rcx_rbx = ((rcx as u128) << 64) | (rbx as u128);

        emu.maps.write_bytes(addr, &rcx_rbx.to_le_bytes());
    } else {
        emu.flags_mut().f_zf = false;

        emu.regs_mut().rax = (mem_val & 0xffffffff_ffffffff) as u64;
        emu.regs_mut().rdx = (mem_val >> 64) as u64;
    }
    true
}
