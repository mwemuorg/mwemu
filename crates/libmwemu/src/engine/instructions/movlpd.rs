use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));

    let sz0 = emu.get_operand_sz(ins, 0);

    if sz0 == 128 {
        // Read a qword explicitly: operand-size decoding can report 32 bits here.
        let value0 = match emu.get_operand_xmm_value_128(ins, 0, false) {
            Some(v) => v,
            None => return false,
        };
        let addr = match emu.get_operand_value(ins, 1, false) {
            Some(v) => v,
            None => {
                log::trace!("error getting movlpd source address");
                return false;
            }
        };
        let value1 = match emu.maps.read_qword(addr) {
            Some(v) => v,
            None => {
                log::trace!("error reading movlpd source qword at 0x{:x}", addr);
                return false;
            }
        };
        let result = (value0 & 0xFFFFFFFFFFFFFFFF_0000000000000000u128) | (value1 as u128);
        emu.set_operand_xmm_value_128(ins, 0, result)
    } else {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };
        let low = value1 as u64;
        let addr = match emu.get_operand_value(ins, 0, false) {
            Some(v) => v,
            None => {
                log::trace!("error getting movlpd destination address");
                return false;
            }
        };
        emu.write_instruction_bytes(addr, &mut low.to_le_bytes())
    }
}
