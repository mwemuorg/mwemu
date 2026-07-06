use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    // movhpd moves the HIGH quadword; the low quadword of the destination is kept.
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz0 = emu.get_operand_sz(ins, 0);

    if sz0 == 128 {
        // `movhpd xmm, m64`: load the HIGH quadword from memory, keep the low half.
        // The memory operand is ALWAYS 64-bit; `get_operand_sz` sometimes
        // misreports it as 32 (which used to hit an `unimplemented!()` panic), so
        // read a qword explicitly from the source address.
        let value0 = match emu.get_operand_xmm_value_128(ins, 0, false) {
            Some(v) => v,
            None => return false,
        };
        let addr = match emu.get_operand_value(ins, 1, false) {
            Some(v) => v,
            None => {
                log::trace!("error getting movhpd source address");
                return false;
            }
        };
        let value1 = match emu.maps.read_qword(addr) {
            Some(v) => v,
            None => {
                log::trace!("error reading movhpd source qword at 0x{:x}", addr);
                return false;
            }
        };
        // keep low 64 bits of the destination, replace the high 64 bits.
        let result = (value0 & 0x0000000000000000_FFFFFFFFFFFFFFFFu128) | ((value1 as u128) << 64);
        emu.set_operand_xmm_value_128(ins, 0, result);
    } else {
        // `movhpd m64, xmm`: store the HIGH quadword of the xmm to memory (64-bit).
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };
        let high = (value1 >> 64) as u64;
        let addr = match emu.get_operand_value(ins, 0, false) {
            Some(v) => v,
            None => {
                log::trace!("error getting movhpd destination address");
                return false;
            }
        };
        emu.maps.write_qword(addr, high);
    }
    true
}
