use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// ADOX dest, src: dest = dest + src + f_of; only f_of is updated.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let s = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let sz = emu.get_operand_sz(ins, 0) as u128;
    let cin = emu.flag_of() as u128;
    let sum = d as u128 + s as u128 + cin;
    let mask = (1u128 << sz) - 1;
    emu.flags_mut().f_of = (sum >> sz) & 1 == 1;
    if !emu.set_operand_value(ins, 0, (sum & mask) as u64) {
        return false;
    }
    true
}
