use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(v) => v,
        None => return false,
    };

    let state = emu.fpu().fxsave();
    // Preserve the register prefix currently modeled by FPUState, but route
    // its CPU store through the same hooks and access checks as SIMD stores.
    let words = [
        state.fpu_control_word as u64
            | ((state.fpu_status_word as u64) << u16::BITS)
            | ((state.fpu_tag_word as u64) << u32::BITS)
            | ((state.fpu_opcode as u64) << (u32::BITS + u16::BITS)),
        state.rip,
        state.rdp,
        state.mxcsr as u64 | ((state.mxcsr_mask as u64) << u32::BITS),
    ];
    let mut bytes: Vec<u8> = words.into_iter().flat_map(u64::to_le_bytes).collect();
    if !emu.write_instruction_bytes(addr, &mut bytes) {
        return false;
    }
    emu.sync_fpu_ip();
    true
}
