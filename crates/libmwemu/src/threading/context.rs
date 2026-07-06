use std::collections::BTreeMap;

use crate::{
    arch::Arch, eflags::Eflags, flags::Flags, fpu::FPU, regs_aarch64::RegsAarch64, regs64::Regs64,
};

/// Architecture-specific per-thread register and exception state.
#[derive(Clone)]
pub enum ArchThreadState {
    X86 {
        regs: Regs64,
        pre_op_regs: Regs64,
        post_op_regs: Regs64,
        flags: Flags,
        pre_op_flags: Flags,
        post_op_flags: Flags,
        eflags: Eflags,
        fpu: FPU,
        seh: u64,
        veh: u64,
        uef: u64,
        eh_ctx: u64,
        tls32: Vec<u32>,
        tls64: Vec<u64>,
        fls: Vec<u32>,
        fs: BTreeMap<u64, u64>,
        call_stack: Vec<(u64, u64)>,
    },
    AArch64 {
        regs: RegsAarch64,
        pre_op_regs: RegsAarch64,
        post_op_regs: RegsAarch64,
    },
}

#[derive(Clone)]
pub struct ThreadContext {
    pub id: u64,                    // Thread ID (e.g., 0x1000, 0x1001, etc.)
    pub suspended: bool,            // Whether thread is suspended
    pub wake_tick: usize,           // Global tick when thread can next run (0 = runnable)
    pub blocked_on_cs: Option<u64>, // Pointer to critical section if blocked
    pub handle: u64,
    pub arch: ArchThreadState,
}

impl ThreadContext {
    pub fn new(id: u64, arch: Arch) -> Self {
        let arch_state = if arch.is_aarch64() {
            ArchThreadState::AArch64 {
                regs: RegsAarch64::new(),
                pre_op_regs: RegsAarch64::new(),
                post_op_regs: RegsAarch64::new(),
            }
        } else {
            ArchThreadState::X86 {
                regs: Regs64::new(),
                pre_op_regs: Regs64::new(),
                post_op_regs: Regs64::new(),
                flags: Flags::new(),
                pre_op_flags: Flags::new(),
                post_op_flags: Flags::new(),
                eflags: Eflags::new(),
                fpu: FPU::new(),
                seh: 0,
                veh: 0,
                uef: 0,
                eh_ctx: 0,
                tls32: Vec::new(),
                tls64: Vec::new(),
                fls: Vec::new(),
                fs: BTreeMap::new(),
                call_stack: Vec::with_capacity(10000),
            }
        };

        ThreadContext {
            id,
            suspended: false,
            wake_tick: 0, // 0 means runnable
            blocked_on_cs: None,
            handle: 0,
            arch: arch_state,
        }
    }
}

// Convenience accessors on ThreadContext for x86 fields
impl ThreadContext {
    #[inline]
    pub fn x86(&self) -> (&Regs64, &Flags, &Eflags, &FPU) {
        match &self.arch {
            ArchThreadState::X86 {
                regs,
                flags,
                eflags,
                fpu,
                ..
            } => (regs, flags, eflags, fpu),
            _ => panic!("x86() called on aarch64 thread"),
        }
    }

    #[inline]
    pub fn regs_x86(&self) -> &Regs64 {
        match &self.arch {
            ArchThreadState::X86 { regs, .. } => regs,
            _ => panic!("regs_x86 called on aarch64 thread"),
        }
    }

    #[inline]
    pub fn regs_x86_mut(&mut self) -> &mut Regs64 {
        match &mut self.arch {
            ArchThreadState::X86 { regs, .. } => regs,
            _ => panic!("regs_x86_mut called on aarch64 thread"),
        }
    }

    #[inline]
    pub fn regs_aarch64(&self) -> &RegsAarch64 {
        match &self.arch {
            ArchThreadState::AArch64 { regs, .. } => regs,
            _ => panic!("regs_aarch64 called on x86 thread"),
        }
    }

    #[inline]
    pub fn regs_aarch64_mut(&mut self) -> &mut RegsAarch64 {
        match &mut self.arch {
            ArchThreadState::AArch64 { regs, .. } => regs,
            _ => panic!("regs_aarch64_mut called on x86 thread"),
        }
    }
}
