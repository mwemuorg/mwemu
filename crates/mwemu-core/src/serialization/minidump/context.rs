use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};
use minidump::format as md;

use crate::arch::Arch;
use crate::arch::aarch64::regs::RegsAarch64;
use crate::flags::Flags;
use crate::regs64::Regs64;

const CONTEXT_X86_SIZE: usize = 716;
const CONTEXT_AMD64_SIZE: usize = 1232;
const CONTEXT_ARM64_SIZE: usize = 912;

pub(super) enum ThreadContextInput<'a> {
    X86 {
        arch: Arch,
        regs: &'a Regs64,
        flags: &'a Flags,
    },
    AArch64 {
        regs: &'a RegsAarch64,
    },
}

pub(super) fn build_thread_context(input: ThreadContextInput<'_>) -> io::Result<Vec<u8>> {
    match input {
        ThreadContextInput::X86 {
            arch: Arch::X86,
            regs,
            flags,
        } => build_x86_context(regs, flags),
        ThreadContextInput::X86 {
            arch: Arch::X86_64,
            regs,
            flags,
        } => build_amd64_context(regs, flags),
        ThreadContextInput::AArch64 { regs } => build_arm64_context(regs),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unexpected architecture for thread context",
        )),
    }
}

fn build_x86_context(regs: &Regs64, flags: &Flags) -> io::Result<Vec<u8>> {
    let mut output = Vec::with_capacity(CONTEXT_X86_SIZE);
    output.write_u32::<LittleEndian>(md::ContextFlagsX86::CONTEXT_X86_ALL.bits())?;
    output.write_u32::<LittleEndian>(regs.dr0 as u32)?;
    output.write_u32::<LittleEndian>(regs.dr1 as u32)?;
    output.write_u32::<LittleEndian>(regs.dr2 as u32)?;
    output.write_u32::<LittleEndian>(regs.dr3 as u32)?;
    output.write_u32::<LittleEndian>(regs.dr6 as u32)?;
    output.write_u32::<LittleEndian>(regs.dr7 as u32)?;

    output.write_u32::<LittleEndian>(0x027f)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(0)?;
    output.extend_from_slice(&[0; 80]);
    output.write_u32::<LittleEndian>(0)?;

    output.write_u32::<LittleEndian>(regs.gs as u32)?;
    output.write_u32::<LittleEndian>(if regs.fs == 0 { 0x3b } else { regs.fs as u32 })?;
    output.write_u32::<LittleEndian>(0x23)?;
    output.write_u32::<LittleEndian>(0x23)?;
    output.write_u32::<LittleEndian>(regs.get_edi() as u32)?;
    output.write_u32::<LittleEndian>(regs.get_esi() as u32)?;
    output.write_u32::<LittleEndian>(regs.get_ebx() as u32)?;
    output.write_u32::<LittleEndian>(regs.get_edx() as u32)?;
    output.write_u32::<LittleEndian>(regs.get_ecx() as u32)?;
    output.write_u32::<LittleEndian>(regs.get_eax() as u32)?;
    output.write_u32::<LittleEndian>(regs.get_ebp() as u32)?;
    output.write_u32::<LittleEndian>(regs.get_eip() as u32)?;
    output.write_u32::<LittleEndian>(0x1b)?;
    output.write_u32::<LittleEndian>(flags.dump())?;
    output.write_u32::<LittleEndian>(regs.get_esp() as u32)?;
    output.write_u32::<LittleEndian>(0x23)?;
    output.extend_from_slice(&[0; 512]);

    debug_assert_eq!(output.len(), CONTEXT_X86_SIZE);
    Ok(output)
}

fn build_amd64_context(regs: &Regs64, flags: &Flags) -> io::Result<Vec<u8>> {
    let mut output = Vec::with_capacity(CONTEXT_AMD64_SIZE);

    for _ in 0..6 {
        output.write_u64::<LittleEndian>(0)?;
    }

    output.write_u32::<LittleEndian>(md::ContextFlagsAmd64::CONTEXT_AMD64_ALL.bits())?;
    output.write_u32::<LittleEndian>(0x1f80)?;
    output.write_u16::<LittleEndian>(0x33)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u16::<LittleEndian>(regs.fs as u16)?;
    output.write_u16::<LittleEndian>(regs.gs as u16)?;
    output.write_u16::<LittleEndian>(0x2b)?;
    output.write_u32::<LittleEndian>(flags.dump())?;
    output.write_u64::<LittleEndian>(regs.dr0)?;
    output.write_u64::<LittleEndian>(regs.dr1)?;
    output.write_u64::<LittleEndian>(regs.dr2)?;
    output.write_u64::<LittleEndian>(regs.dr3)?;
    output.write_u64::<LittleEndian>(regs.dr6)?;
    output.write_u64::<LittleEndian>(regs.dr7)?;
    output.write_u64::<LittleEndian>(regs.rax)?;
    output.write_u64::<LittleEndian>(regs.rcx)?;
    output.write_u64::<LittleEndian>(regs.rdx)?;
    output.write_u64::<LittleEndian>(regs.rbx)?;
    output.write_u64::<LittleEndian>(regs.rsp)?;
    output.write_u64::<LittleEndian>(regs.rbp)?;
    output.write_u64::<LittleEndian>(regs.rsi)?;
    output.write_u64::<LittleEndian>(regs.rdi)?;
    output.write_u64::<LittleEndian>(regs.r8)?;
    output.write_u64::<LittleEndian>(regs.r9)?;
    output.write_u64::<LittleEndian>(regs.r10)?;
    output.write_u64::<LittleEndian>(regs.r11)?;
    output.write_u64::<LittleEndian>(regs.r12)?;
    output.write_u64::<LittleEndian>(regs.r13)?;
    output.write_u64::<LittleEndian>(regs.r14)?;
    output.write_u64::<LittleEndian>(regs.r15)?;
    output.write_u64::<LittleEndian>(regs.rip)?;

    write_xmm_save_area32(&mut output, regs)?;

    for _ in 0..26 {
        write_u128(&mut output, 0)?;
    }

    for _ in 0..6 {
        output.write_u64::<LittleEndian>(0)?;
    }

    debug_assert_eq!(output.len(), CONTEXT_AMD64_SIZE);
    Ok(output)
}

fn build_arm64_context(regs: &RegsAarch64) -> io::Result<Vec<u8>> {
    let mut output = Vec::with_capacity(CONTEXT_ARM64_SIZE);

    // context_flags (u32)
    output.write_u32::<LittleEndian>(md::ContextFlagsArm64::CONTEXT_ARM64_ALL.bits())?;

    // cpsr (u32) - NZCV flags packed into bits [31:28]
    output.write_u32::<LittleEndian>(regs.nzcv.as_u64() as u32)?;

    // iregs[31] (x0-x28, fp=x29, lr=x30)
    for i in 0..31 {
        output.write_u64::<LittleEndian>(regs.x[i])?;
    }

    // sp (u64)
    output.write_u64::<LittleEndian>(regs.sp)?;

    // pc (u64)
    output.write_u64::<LittleEndian>(regs.pc)?;

    // float_regs[32] (V0-V31 as u128)
    for i in 0..32 {
        write_u128(&mut output, regs.v[i])?;
    }

    // fpcr (u32)
    output.write_u32::<LittleEndian>(regs.fpcr as u32)?;

    // fpsr (u32)
    output.write_u32::<LittleEndian>(regs.fpsr as u32)?;

    // bcr[8] (u32 each) - breakpoint control registers
    for _ in 0..8 {
        output.write_u32::<LittleEndian>(0)?;
    }

    // bvr[8] (u64 each) - breakpoint value registers
    for _ in 0..8 {
        output.write_u64::<LittleEndian>(0)?;
    }

    // wcr[2] (u32 each) - watchpoint control registers
    for _ in 0..2 {
        output.write_u32::<LittleEndian>(0)?;
    }

    // wvr[2] (u64 each) - watchpoint value registers
    for _ in 0..2 {
        output.write_u64::<LittleEndian>(0)?;
    }

    debug_assert_eq!(output.len(), CONTEXT_ARM64_SIZE);
    Ok(output)
}

fn write_xmm_save_area32(output: &mut Vec<u8>, regs: &Regs64) -> io::Result<()> {
    output.write_u16::<LittleEndian>(0x027f)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u8(0)?;
    output.write_u8(0)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(0)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u16::<LittleEndian>(0)?;
    output.write_u32::<LittleEndian>(0x1f80)?;
    output.write_u32::<LittleEndian>(0xffff)?;

    for mm in [
        regs.mm0, regs.mm1, regs.mm2, regs.mm3, regs.mm4, regs.mm5, regs.mm6, regs.mm7,
    ] {
        write_u128(output, mm)?;
    }

    for xmm in [
        regs.xmm0, regs.xmm1, regs.xmm2, regs.xmm3, regs.xmm4, regs.xmm5, regs.xmm6, regs.xmm7,
        regs.xmm8, regs.xmm9, regs.xmm10, regs.xmm11, regs.xmm12, regs.xmm13, regs.xmm14,
        regs.xmm15,
    ] {
        write_u128(output, xmm)?;
    }

    output.extend_from_slice(&[0; 96]);
    Ok(())
}

fn write_u128(output: &mut Vec<u8>, value: u128) -> io::Result<()> {
    output.write_all(&value.to_le_bytes())
}
