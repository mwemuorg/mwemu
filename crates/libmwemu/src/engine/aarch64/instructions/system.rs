use super::super::helpers::{read_operand_value, write_reg};
use crate::emu::Emu;
use crate::windows::constants::memory::BLOCK_LEN;
use yaxpeax_arm::armv8::a64::{Instruction, Operand, SysOps};

const TPIDR_EL0: u16 = 0x5e82;
const CTR_EL0: u16 = 0x5801;
// CTR_EL0 encodes log2(words per line) in DminLine and IminLine.
const LOG2_CACHE_LINE_WORDS: u32 = 4;
const CACHE_LINE_BYTES: u64 = 4 << LOG2_CACHE_LINE_WORDS;
const CACHE_TYPE: u64 = (LOG2_CACHE_LINE_WORDS as u64) << 16 | LOG2_CACHE_LINE_WORDS as u64;

pub fn read(emu: &mut Emu, ins: &Instruction) -> bool {
    let value = match ins.operands[1] {
        Operand::SystemReg(TPIDR_EL0) => emu.regs_aarch64().tpidr_el0,
        Operand::SystemReg(CTR_EL0) => CACHE_TYPE,
        _ => 0,
    };
    write_reg(emu, &ins.operands[0], value);
    true
}

pub fn write(emu: &mut Emu, ins: &Instruction) -> bool {
    if ins.operands[0] == Operand::SystemReg(TPIDR_EL0) {
        emu.regs_aarch64_mut().tpidr_el0 = read_operand_value(emu, &ins.operands[1]);
    }
    true
}

pub fn cache_maintenance(emu: &mut Emu, ins: &Instruction, ops: SysOps) -> bool {
    let [reg, Operand::ControlReg(7), Operand::ControlReg(cache), _] = ins.operands else {
        return false;
    };
    if (ops.op1(), ops.op2()) != (3, 1) {
        return false;
    }
    let addr = read_operand_value(emu, &reg);
    if !emu.maps.is_mapped(addr) {
        return false;
    }
    match cache {
        11 => true, // DC CVAU: guest memory is already coherent.
        5 => {
            // IC IVAU
            let line = addr & !(CACHE_LINE_BYTES - 1);
            emu.aarch64_instruction_cache().invalidate_range(
                line,
                CACHE_LINE_BYTES,
                BLOCK_LEN as u64,
            );
            true
        }
        _ => false,
    }
}
