use iced_x86::{Instruction, Mnemonic, Register};

use crate::arch::x86::{
    XCR0_ENABLED_MASK, XSAVE_AREA_BYTES, XSAVE_AVX, XSAVE_AVX_OFFSET, XSAVE_SSE, XSAVE_X87,
};
use crate::color;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};
use crate::exception::types::ExceptionType;
use crate::fpu::FPU;

const ALIGNMENT: u64 = 64;
const HEADER: u64 = 512;
const HEADER_BYTES: usize = 64;
const MXCSR: u64 = 24;
const MXCSR_MASK: u32 = 0xffff;
const XMM: u64 = 160;
const SLOT: usize = size_of::<u128>();
const X87_REGISTERS: usize = 8;
const X87_SLOTS: usize = 32;
const X87_ENV: usize = 24;
const X87_REGISTER_BYTES: usize = 10;
const VECTOR_BYTES: usize = 16 * SLOT;

fn field<const N: usize>(bytes: &[u8], offset: usize) -> [u8; N] {
    let mut result = [0; N];
    result.copy_from_slice(&bytes[offset..offset + N]);
    result
}

fn x87_env(fpu: &FPU, wide: bool) -> [u8; X87_ENV] {
    let mut env = [0; X87_ENV];
    env[0..2].copy_from_slice(&fpu.get_ctrl().to_le_bytes());
    env[2..4].copy_from_slice(&fpu.status_word().to_le_bytes());
    env[4] = fpu.st.abridged_tag();
    env[6..8].copy_from_slice(&fpu.opcode.to_le_bytes());
    if wide {
        env[8..16].copy_from_slice(&fpu.ip.to_le_bytes());
        env[16..24].copy_from_slice(&fpu.operand_ptr.to_le_bytes());
    } else {
        env[8..12].copy_from_slice(&(fpu.ip as u32).to_le_bytes());
        env[12..14].copy_from_slice(&fpu.code_segment.to_le_bytes());
        env[16..20].copy_from_slice(&(fpu.operand_ptr as u32).to_le_bytes());
        env[20..22].copy_from_slice(&fpu.data_segment.to_le_bytes());
    }
    env
}

fn restore_x87(fpu: &mut FPU, bytes: &[u8; XMM as usize], wide: bool) {
    fpu.ctrl = u16::from_le_bytes(field(bytes, 0));
    fpu.status = u16::from_le_bytes(field(bytes, 2));
    fpu.fpu_control_word = fpu.ctrl;
    fpu.stat = fpu.status;
    fpu.opcode = u16::from_le_bytes(field(bytes, 6));
    if wide {
        fpu.ip = u64::from_le_bytes(field(bytes, 8));
        fpu.operand_ptr = u64::from_le_bytes(field(bytes, 16));
    } else {
        fpu.ip = u32::from_le_bytes(field(bytes, 8)) as u64;
        fpu.code_segment = u16::from_le_bytes(field(bytes, 12));
        fpu.operand_ptr = u32::from_le_bytes(field(bytes, 16)) as u64;
        fpu.data_segment = u16::from_le_bytes(field(bytes, 20));
    }
    let regs = std::array::from_fn(|i| {
        let mut bits = [0; SLOT];
        let offset = X87_SLOTS + SLOT * i;
        bits[..X87_REGISTER_BYTES].copy_from_slice(&bytes[offset..offset + X87_REGISTER_BYTES]);
        u128::from_le_bytes(bits)
    });
    fpu.tag = fpu
        .st
        .restore_image(((fpu.status >> 11) & 7) as u8, bytes[4], regs);
    fpu.st_depth = fpu.st.get_depth();
}

fn invalid(emu: &mut Emu) -> bool {
    emu.exception(ExceptionType::BadAddressDereferencing);
    false
}

fn load<const N: usize>(emu: &mut Emu, addr: u64, len: usize) -> Option<[u8; N]> {
    emu.read_instruction_bytes(addr, len, ExceptionType::ReadingXmmOperand)
}

fn save(emu: &mut Emu, addr: u64, mask: u64, count: usize, wide: bool) -> bool {
    let Some(header) = load::<8>(emu, addr + HEADER, 8) else {
        return false;
    };
    if mask & XSAVE_X87 != 0 {
        let mut env = x87_env(emu.fpu(), wide);
        let mut registers = [0; X87_REGISTERS * SLOT];
        let stack = &emu.fpu().st;
        for (i, slot) in registers.chunks_exact_mut(SLOT).enumerate() {
            let raw = stack.peek(stack.physical_index(i)).get().to_le_bytes();
            slot[..X87_REGISTER_BYTES].copy_from_slice(&raw[..X87_REGISTER_BYTES]);
        }
        if !emu.write_instruction_bytes(addr, &mut env)
            || !emu.write_instruction_bytes(addr + X87_SLOTS as u64, &mut registers)
        {
            return false;
        }
    }
    if mask & (XSAVE_SSE | XSAVE_AVX) != 0 {
        let raw = emu.fpu().mxcsr as u64 | ((MXCSR_MASK as u64) << u32::BITS);
        if !emu.write_instruction_bytes(addr + MXCSR, &mut raw.to_le_bytes()) {
            return false;
        }
    }
    for (feature, offset) in [(XSAVE_SSE, XMM), (XSAVE_AVX, XSAVE_AVX_OFFSET)] {
        if mask & feature == 0 {
            continue;
        }
        let mut bytes = [0; VECTOR_BYTES];
        for (i, slot) in bytes[..count * SLOT].chunks_exact_mut(SLOT).enumerate() {
            let value = if feature == XSAVE_SSE {
                emu.regs().get_xmm_reg(Register::XMM0 + i as u32)
            } else {
                let ymm = emu.regs().get_ymm_reg(Register::YMM0 + i as u32);
                ymm.0[2] as u128 | ((ymm.0[3] as u128) << 64)
            };
            slot.copy_from_slice(&value.to_le_bytes());
        }
        if !emu.write_instruction_bytes(addr + offset, &mut bytes[..count * SLOT]) {
            return false;
        }
    }
    let state_bv = u64::from_le_bytes(header) | mask;
    emu.write_instruction_bytes(addr + HEADER, &mut state_bv.to_le_bytes())
}

fn restore(emu: &mut Emu, addr: u64, mask: u64, count: usize, wide: bool) -> bool {
    let Some(header) = load::<HEADER_BYTES>(emu, addr + HEADER, HEADER_BYTES) else {
        return false;
    };
    let present = u64::from_le_bytes(field(&header, 0));
    if present & !XCR0_ENABLED_MASK != 0 || header[8..24].iter().any(|&b| b != 0) {
        return invalid(emu);
    }
    let mut x87 = [0; XMM as usize];
    if mask & XSAVE_X87 != 0 {
        if present & XSAVE_X87 != 0 {
            let Some(bytes) = load(emu, addr, x87.len()) else {
                return false;
            };
            x87 = bytes;
        } else {
            x87[..2].copy_from_slice(&0x037fu16.to_le_bytes());
        }
    }
    let mut mxcsr = emu.fpu().mxcsr;
    if mask & (XSAVE_SSE | XSAVE_AVX) != 0 {
        let Some(bytes) = load::<4>(emu, addr + MXCSR, 4) else {
            return false;
        };
        mxcsr = u32::from_le_bytes(bytes);
        if mxcsr & !MXCSR_MASK != 0 {
            return invalid(emu);
        }
    }
    let mut xmm = [0; VECTOR_BYTES];
    let mut high = [0; VECTOR_BYTES];
    for (feature, offset, dest) in [
        (XSAVE_SSE, XMM, &mut xmm),
        (XSAVE_AVX, XSAVE_AVX_OFFSET, &mut high),
    ] {
        if mask & present & feature != 0 {
            let Some(bytes) = load(emu, addr + offset, count * SLOT) else {
                return false;
            };
            *dest = bytes;
        }
    }
    if mask & XSAVE_X87 != 0 {
        restore_x87(emu.fpu_mut(), &x87, wide);
    }
    emu.fpu_mut().mxcsr = mxcsr;
    for i in 0..count {
        if mask & XSAVE_SSE != 0 {
            emu.regs_mut().set_xmm_reg(
                Register::XMM0 + i as u32,
                u128::from_le_bytes(field(&xmm, i * SLOT)),
            );
        }
        if mask & XSAVE_AVX != 0 {
            let register = Register::YMM0 + i as u32;
            let mut value = emu.regs().get_ymm_reg(register);
            value.0[2] = u64::from_le_bytes(field(&high, i * SLOT));
            value.0[3] = u64::from_le_bytes(field(&high, i * SLOT + 8));
            emu.regs_mut().set_ymm_reg(register, value);
        }
    }
    true
}

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let Some(addr) = emu.get_operand_value(ins, 0, false) else {
        return false;
    };
    if !addr.is_multiple_of(ALIGNMENT) || addr.checked_add(XSAVE_AREA_BYTES).is_none() {
        return invalid(emu);
    }
    let mask = (emu.regs().get_eax() | (emu.regs().get_edx() << 32)) & XCR0_ENABLED_MASK;
    let count = if emu.cfg.is_x64() { 16 } else { 8 };
    let wide = matches!(ins.mnemonic(), Mnemonic::Xsave64 | Mnemonic::Xrstor64);
    if matches!(ins.mnemonic(), Mnemonic::Xsave | Mnemonic::Xsave64) {
        save(emu, addr, mask, count, wide)
    } else {
        restore(emu, addr, mask, count, wide)
    }
}
