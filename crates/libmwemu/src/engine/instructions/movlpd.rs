use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    // movlpd moves the LOW quadword; the high quadword of the destination is kept.
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz0 = emu.get_operand_sz(ins, 0);

    if sz0 == 128 {
        // `movlpd xmm, m64`: load the LOW quadword from memory, keep the high half.
        // The memory operand is ALWAYS 64-bit; `get_operand_sz` sometimes
        // misreports it as 32, which previously read only 32 bits AND wiped the
        // high half. Read a qword explicitly from the source address.
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
        // keep high 64 bits of the destination, replace the low 64 bits.
        let result = (value0 & 0xFFFFFFFFFFFFFFFF_0000000000000000u128) | (value1 as u128);
        emu.set_operand_xmm_value_128(ins, 0, result);
    } else {
        // `movlpd m64, xmm`: store the LOW quadword of the xmm to memory (64-bit).
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
        emu.maps.write_qword(addr, low);
    }
    true
}
