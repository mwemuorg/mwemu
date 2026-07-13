use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// BEXTR dest, src, control: start = control[7:0], len = control[15:8];
// dest = (src >> start) & ((1 << len) - 1)
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let control = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    let opmask: u64 = if sz >= 64 { u64::MAX } else { (1u64 << sz) - 1 };
    let start = (control & 0xff) as u32;
    let len = ((control >> 8) & 0xff) as u32;

    let shifted = if start >= 64 {
        0
    } else {
        (src & opmask) >> start
    };
    let result = if len >= 64 {
        shifted
    } else {
        shifted & ((1u64 << len) - 1)
    } & opmask;

    // ZF reflects the result; CF and OF are cleared (AF/SF/PF undefined).
    emu.flags_mut().f_zf = result == 0;
    emu.flags_mut().f_cf = false;
    emu.flags_mut().f_of = false;

    emu.set_operand_value(ins, 0, result);
    true
}
