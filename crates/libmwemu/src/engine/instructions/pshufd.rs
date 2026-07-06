use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let source = emu
        .get_operand_xmm_value_128(ins, 1, true)
        .expect("error getting source");
    let order = emu
        .get_operand_value(ins, 2, true)
        .expect("error getting order");

    let order1 = get_bit!(order, 0) | (get_bit!(order, 1) << 1);
    let order2 = get_bit!(order, 2) | (get_bit!(order, 3) << 1);
    let order3 = get_bit!(order, 4) | (get_bit!(order, 5) << 1);
    let order4 = get_bit!(order, 6) | (get_bit!(order, 7) << 1);

    let mut dest: u128 = (source >> (order1 * 32)) as u32 as u128;
    dest |= ((source >> (order2 * 32)) as u32 as u128) << 32;
    dest |= ((source >> (order3 * 32)) as u32 as u128) << 64;
    dest |= ((source >> (order4 * 32)) as u32 as u128) << 96;

    emu.set_operand_xmm_value_128(ins, 0, dest);
    true
}
