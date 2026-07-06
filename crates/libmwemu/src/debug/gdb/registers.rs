//! Register mapping between mwemu and gdbstub
//!
//! This module handles the conversion between mwemu's register representation
//! and the GDB protocol's register format.

use gdbstub_arch::aarch64::reg::AArch64CoreRegs;
use gdbstub_arch::x86::reg::{X86CoreRegs, X86SegmentRegs, X87FpuInternalRegs};

use crate::emu::Emu;

/// Read AArch64 registers from mwemu into gdbstub's AArch64CoreRegs format
pub fn read_regs_aarch64(emu: &Emu) -> AArch64CoreRegs {
    let regs = emu.regs_aarch64();
    let mut gdb_regs = AArch64CoreRegs::default();
    gdb_regs.x = regs.x;
    gdb_regs.sp = regs.sp;
    gdb_regs.pc = regs.pc;
    gdb_regs.cpsr = regs.nzcv.as_u64() as u32;
    gdb_regs.v = regs.v;
    gdb_regs.fpcr = regs.fpcr as u32;
    gdb_regs.fpsr = regs.fpsr as u32;
    gdb_regs
}

/// Write AArch64 registers from gdbstub's format back to mwemu
pub fn write_regs_aarch64(emu: &mut Emu, gdb_regs: &AArch64CoreRegs) {
    let regs = emu.regs_aarch64_mut();
    regs.x = gdb_regs.x;
    regs.sp = gdb_regs.sp;
    regs.pc = gdb_regs.pc;
    regs.nzcv.from_u64(gdb_regs.cpsr as u64);
    regs.v = gdb_regs.v;
    regs.fpcr = gdb_regs.fpcr as u64;
    regs.fpsr = gdb_regs.fpsr as u64;
}

/// Read 64-bit registers from mwemu into gdbstub's X86_64CoreRegs format
pub fn read_regs_64(emu: &Emu) -> gdbstub_arch::x86::reg::X86_64CoreRegs {
    let regs = emu.regs();
    let flags = emu.flags_snapshot();
    let fpu = emu.fpu();

    let mut gdb_regs = gdbstub_arch::x86::reg::X86_64CoreRegs::default();

    // General purpose registers
    // Order: rax, rbx, rcx, rdx, rsi, rdi, rbp, rsp, r8-r15
    gdb_regs.regs = [
        regs.rax, regs.rbx, regs.rcx, regs.rdx, regs.rsi, regs.rdi, regs.rbp, regs.rsp, regs.r8,
        regs.r9, regs.r10, regs.r11, regs.r12, regs.r13, regs.r14, regs.r15,
    ];

    // Instruction pointer
    gdb_regs.rip = regs.rip;

    // Flags - use dump() to get the raw u32 value
    gdb_regs.eflags = flags.dump();

    // Segment registers - use typical Windows x64 values
    gdb_regs.segments = gdbstub_arch::x86::reg::X86SegmentRegs {
        cs: 0x33, // 64-bit code segment
        ss: 0x2b, // Stack segment
        ds: 0x2b, // Data segment
        es: 0x2b, // Extra segment
        fs: 0x53, // FS segment (used for TLS on Windows)
        gs: 0x2b, // GS segment
    };

    // ST registers (FPU) - 80-bit extended precision, stored as [u8; 10]
    for i in 0..8 {
        let st_val = fpu.st.peek(i).get();
        // Convert u128 to [u8; 10] for 80-bit float
        let bytes = st_val.to_le_bytes();
        gdb_regs.st[i] = [
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9],
        ];
    }

    // FPU internal registers
    gdb_regs.fpu = X87FpuInternalRegs {
        fctrl: fpu.ctrl as u32,
        fstat: fpu.stat as u32,
        ftag: fpu.tag as u32,
        fiseg: 0,
        fioff: fpu.ip as u32,
        foseg: 0,
        fooff: fpu.operand_ptr as u32,
        fop: fpu.opcode as u32,
    };

    // XMM registers
    for i in 0..16 {
        let xmm_val = match i {
            0 => regs.xmm0,
            1 => regs.xmm1,
            2 => regs.xmm2,
            3 => regs.xmm3,
            4 => regs.xmm4,
            5 => regs.xmm5,
            6 => regs.xmm6,
            7 => regs.xmm7,
            8 => regs.xmm8,
            9 => regs.xmm9,
            10 => regs.xmm10,
            11 => regs.xmm11,
            12 => regs.xmm12,
            13 => regs.xmm13,
            14 => regs.xmm14,
            15 => regs.xmm15,
            _ => 0,
        };
        gdb_regs.xmm[i] = xmm_val;
    }

    // MXCSR register
    gdb_regs.mxcsr = fpu.mxcsr;

    gdb_regs
}

/// Write 64-bit registers from gdbstub's format back to mwemu
pub fn write_regs_64(emu: &mut Emu, gdb_regs: &gdbstub_arch::x86::reg::X86_64CoreRegs) {
    let regs = emu.regs_mut();

    // General purpose registers
    regs.rax = gdb_regs.regs[0];
    regs.rbx = gdb_regs.regs[1];
    regs.rcx = gdb_regs.regs[2];
    regs.rdx = gdb_regs.regs[3];
    regs.rsi = gdb_regs.regs[4];
    regs.rdi = gdb_regs.regs[5];
    regs.rbp = gdb_regs.regs[6];
    regs.rsp = gdb_regs.regs[7];
    regs.r8 = gdb_regs.regs[8];
    regs.r9 = gdb_regs.regs[9];
    regs.r10 = gdb_regs.regs[10];
    regs.r11 = gdb_regs.regs[11];
    regs.r12 = gdb_regs.regs[12];
    regs.r13 = gdb_regs.regs[13];
    regs.r14 = gdb_regs.regs[14];
    regs.r15 = gdb_regs.regs[15];

    // Instruction pointer
    regs.rip = gdb_regs.rip;

    // XMM registers
    regs.xmm0 = gdb_regs.xmm[0];
    regs.xmm1 = gdb_regs.xmm[1];
    regs.xmm2 = gdb_regs.xmm[2];
    regs.xmm3 = gdb_regs.xmm[3];
    regs.xmm4 = gdb_regs.xmm[4];
    regs.xmm5 = gdb_regs.xmm[5];
    regs.xmm6 = gdb_regs.xmm[6];
    regs.xmm7 = gdb_regs.xmm[7];
    regs.xmm8 = gdb_regs.xmm[8];
    regs.xmm9 = gdb_regs.xmm[9];
    regs.xmm10 = gdb_regs.xmm[10];
    regs.xmm11 = gdb_regs.xmm[11];
    regs.xmm12 = gdb_regs.xmm[12];
    regs.xmm13 = gdb_regs.xmm[13];
    regs.xmm14 = gdb_regs.xmm[14];
    regs.xmm15 = gdb_regs.xmm[15];

    // Flags
    emu.flags_mut().load(gdb_regs.eflags);

    // FPU state
    let fpu = emu.fpu_mut();
    fpu.ctrl = gdb_regs.fpu.fctrl as u16;
    fpu.stat = gdb_regs.fpu.fstat as u16;
    fpu.tag = gdb_regs.fpu.ftag as u16;
    fpu.ip = gdb_regs.fpu.fioff as u64;
    fpu.operand_ptr = gdb_regs.fpu.fooff as u64;
    fpu.opcode = gdb_regs.fpu.fop as u16;
    fpu.mxcsr = gdb_regs.mxcsr;

    // ST registers
    for i in 0..8 {
        let bytes = gdb_regs.st[i];
        let mut val_bytes = [0u8; 16];
        val_bytes[..10].copy_from_slice(&bytes);
        let val = u128::from_le_bytes(val_bytes);
        fpu.set_st_u80(i, val);
    }
}

/// Read 32-bit registers from mwemu into gdbstub's X86CoreRegs format
pub fn read_regs_32(emu: &Emu) -> X86CoreRegs {
    let regs = emu.regs();
    let flags = emu.flags_snapshot();
    let fpu = emu.fpu();

    let mut gdb_regs = X86CoreRegs::default();

    // General purpose registers (32-bit)
    gdb_regs.eax = regs.rax as u32;
    gdb_regs.ecx = regs.rcx as u32;
    gdb_regs.edx = regs.rdx as u32;
    gdb_regs.ebx = regs.rbx as u32;
    gdb_regs.esp = regs.rsp as u32;
    gdb_regs.ebp = regs.rbp as u32;
    gdb_regs.esi = regs.rsi as u32;
    gdb_regs.edi = regs.rdi as u32;

    // Instruction pointer
    gdb_regs.eip = regs.rip as u32;

    // Flags
    gdb_regs.eflags = flags.dump();

    // Segment registers - use typical Windows x86 values
    gdb_regs.segments = X86SegmentRegs {
        cs: 0x1b, // 32-bit code segment
        ss: 0x23, // Stack segment
        ds: 0x23, // Data segment
        es: 0x23, // Extra segment
        fs: 0x3b, // FS segment (used for TEB on Windows)
        gs: 0x00, // GS segment
    };

    // ST registers (FPU)
    for i in 0..8 {
        let st_val = fpu.st.peek(i).get();
        let bytes = st_val.to_le_bytes();
        gdb_regs.st[i] = [
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9],
        ];
    }

    // FPU internal registers
    gdb_regs.fpu = X87FpuInternalRegs {
        fctrl: fpu.ctrl as u32,
        fstat: fpu.stat as u32,
        ftag: fpu.tag as u32,
        fiseg: 0,
        fioff: fpu.ip as u32,
        foseg: 0,
        fooff: fpu.operand_ptr as u32,
        fop: fpu.opcode as u32,
    };

    // XMM registers (only 8 for 32-bit)
    for i in 0..8 {
        let xmm_val = match i {
            0 => regs.xmm0,
            1 => regs.xmm1,
            2 => regs.xmm2,
            3 => regs.xmm3,
            4 => regs.xmm4,
            5 => regs.xmm5,
            6 => regs.xmm6,
            7 => regs.xmm7,
            _ => 0,
        };
        gdb_regs.xmm[i] = xmm_val;
    }

    // MXCSR register
    gdb_regs.mxcsr = fpu.mxcsr;

    gdb_regs
}

/// Write 32-bit registers from gdbstub's format back to mwemu
pub fn write_regs_32(emu: &mut Emu, gdb_regs: &X86CoreRegs) {
    let regs = emu.regs_mut();

    // General purpose registers (keep upper 32 bits cleared)
    regs.rax = gdb_regs.eax as u64;
    regs.rcx = gdb_regs.ecx as u64;
    regs.rdx = gdb_regs.edx as u64;
    regs.rbx = gdb_regs.ebx as u64;
    regs.rsp = gdb_regs.esp as u64;
    regs.rbp = gdb_regs.ebp as u64;
    regs.rsi = gdb_regs.esi as u64;
    regs.rdi = gdb_regs.edi as u64;

    // Instruction pointer
    regs.rip = gdb_regs.eip as u64;

    // XMM registers
    regs.xmm0 = gdb_regs.xmm[0];
    regs.xmm1 = gdb_regs.xmm[1];
    regs.xmm2 = gdb_regs.xmm[2];
    regs.xmm3 = gdb_regs.xmm[3];
    regs.xmm4 = gdb_regs.xmm[4];
    regs.xmm5 = gdb_regs.xmm[5];
    regs.xmm6 = gdb_regs.xmm[6];
    regs.xmm7 = gdb_regs.xmm[7];

    // Flags
    emu.flags_mut().load(gdb_regs.eflags);

    // FPU state
    let fpu = emu.fpu_mut();
    fpu.ctrl = gdb_regs.fpu.fctrl as u16;
    fpu.stat = gdb_regs.fpu.fstat as u16;
    fpu.tag = gdb_regs.fpu.ftag as u16;
    fpu.ip = gdb_regs.fpu.fioff as u64;
    fpu.operand_ptr = gdb_regs.fpu.fooff as u64;
    fpu.opcode = gdb_regs.fpu.fop as u16;
    fpu.mxcsr = gdb_regs.mxcsr;

    // ST registers
    for i in 0..8 {
        let bytes = gdb_regs.st[i];
        let mut val_bytes = [0u8; 16];
        val_bytes[..10].copy_from_slice(&bytes);
        let val = u128::from_le_bytes(val_bytes);
        fpu.set_st_u80(i, val);
    }
}
