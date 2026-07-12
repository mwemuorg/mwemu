use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    // we keep the high part of xmm destination

    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    if sz0 == 128 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
        emu.set_operand_xmm_value_128(ins, 0, value1);
    } else if sz0 == 128 && sz1 == 32 {
        let src = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v as i32,
            None => {
                log::trace!("error getting cvtsi2ss src32");
                return false;
            }
        };
        let dst = match emu.get_operand_xmm_value_128(ins, 0, true) {
            Some(v) => v,
            None => return false,
        };
        // Convert the signed integer to f32 in bits [31:0]; leave [127:32] intact.
        let bits = (src as f32).to_bits() as u128;
        emu.set_operand_xmm_value_128(ins, 0, (dst & !0xFFFF_FFFFu128) | bits);
    } else if sz0 == 32 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
        emu.set_operand_value(ins, 0, value1 as u64);
    } else if sz0 == 128 && sz1 == 64 {
        let src = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v as i64,
            None => {
                log::trace!("error getting cvtsi2ss src64");
                return false;
            }
        };
        let dst = match emu.get_operand_xmm_value_128(ins, 0, true) {
            Some(v) => v,
            None => return false,
        };
        // Convert the signed 64-bit integer to f32 in bits [31:0]; keep [127:32].
        let bits = (src as f32).to_bits() as u128;
        emu.set_operand_xmm_value_128(ins, 0, (dst & !0xFFFF_FFFFu128) | bits);
    } else if sz0 == 64 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::trace!("error getting xmm value1");
                return false;
            }
        };
        emu.set_operand_value(ins, 0, value1 as u64);
    } else {
        log::trace!("SSE with other size combinations sz0:{} sz1:{}", sz0, sz1);
        return false;
    }
    true
}
