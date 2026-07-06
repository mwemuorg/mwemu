use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    assert!(ins.op_count() == 2);

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    if sz0 == 32 && sz1 == 128 {
        let xmm = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("{} error getting xmm value1", emu.pos);
                return false;
            }
        };
        let addr = match emu.get_operand_value(ins, 0, false) {
            Some(v) => v,
            None => {
                log::trace!("{} error getting address value0", emu.pos);
                return false;
            }
        };
        // movdqa stores in little-endian: the low 32 bits of xmm go to addr+0,
        // ..., the high 32 bits go to addr+12. (The previous implementation
        // wrote the dwords reversed, which silently corrupted any UNICODE_STRING
        // or other 16-byte struct passed through xmm registers — surfaced
        // during ntdll's `LdrpInitializeProcess` BaseDllName setup.)
        emu.maps.write_dword(addr, (xmm & 0xffffffff) as u32);
        emu.maps
            .write_dword(addr + 4, ((xmm >> 32) & 0xffffffff) as u32);
        emu.maps
            .write_dword(addr + 8, ((xmm >> 64) & 0xffffffff) as u32);
        emu.maps
            .write_dword(addr + 12, ((xmm >> 96) & 0xffffffff) as u32);
    } else if sz0 == 128 && sz1 == 32 {
        let addr = match emu.get_operand_value(ins, 1, false) {
            Some(v) => v,
            None => {
                log::trace!("{} error reading address value1", emu.pos);
                return false;
            }
        };

        let bytes = emu.maps.read_bytes(addr, 16);
        if bytes.len() != 16 {
            log::trace!("{} error reading 16 bytes at {addr:x}", emu.pos);
            return false;
        }

        let result = u128::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]);

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else if sz0 == 128 && sz1 == 128 {
        let xmm = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("{} error getting xmm value1", emu.pos);
                return false;
            }
        };

        emu.set_operand_xmm_value_128(ins, 0, xmm);
    } else {
        log::trace!("{} sz0: {}  sz1: {}\n", emu.pos, sz0, sz1);
        unimplemented!("movdqa");
    }
    true
}
