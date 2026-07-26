use std::collections::BTreeMap;

use crate::{
    arch::Arch, eflags::Eflags, flags::Flags, fpu::FPU, regs_aarch64::RegsAarch64, regs64::Regs64,
};

/// x86 trace snapshot: pre/post register and flag copies for `--trace-regs`.
/// Stored as `Box<T>` so the variant can stay zero-sized in the common
/// no-trace case (an `Option<Box<T>>` is the size of one pointer).
#[derive(Clone)]
pub struct X86TraceSnapshot {
    pub pre_regs: Regs64,
    pub post_regs: Regs64,
    pub pre_flags: Flags,
    pub post_flags: Flags,
}

impl X86TraceSnapshot {
    pub fn new() -> Self {
        Self {
            pre_regs: Regs64::new(),
            post_regs: Regs64::new(),
            pre_flags: Flags::new(),
            post_flags: Flags::new(),
        }
    }
}

/// AArch64 trace snapshot: pre/post register copies for `--trace-regs`.
#[derive(Clone)]
pub struct AArch64TraceSnapshot {
    pub pre_regs: RegsAarch64,
    pub post_regs: RegsAarch64,
}

impl AArch64TraceSnapshot {
    pub fn new() -> Self {
        Self {
            pre_regs: RegsAarch64::new(),
            post_regs: RegsAarch64::new(),
        }
    }
}

/// Architecture-specific per-thread register and exception state.
///
/// Trace pre/post snapshots (`x86_trace` / `aarch64_trace`) are
/// `Option<Box<…>>` so a run without `--trace-regs` skips the per-thread
/// buffer allocation entirely.
#[derive(Clone)]
pub enum ArchThreadState {
    X86 {
        regs: Regs64,
        flags: Flags,
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
        /// Lazily-allocated trace snapshot. `None` until first capture.
        x86_trace: Option<Box<X86TraceSnapshot>>,
    },
    AArch64 {
        regs: RegsAarch64,
        /// Lazily-allocated trace snapshot. `None` until first capture.
        aarch64_trace: Option<Box<AArch64TraceSnapshot>>,
    },
}


impl ArchThreadState {
    /// Return a mutable reference to the lazily-allocated x86 trace
    /// snapshot. Always `Some` at runtime; panics on aarch64.
    #[inline]
    pub fn x86_trace_mut(&mut self) -> &mut X86TraceSnapshot {
        let ArchThreadState::X86 { x86_trace, .. } = self else {
            unreachable!("x86_trace_mut called on aarch64 thread");
        };
        x86_trace
            .as_mut()
            .expect("x86 trace snapshot was set at construction")
    }

    /// Return an immutable reference to the x86 trace snapshot.
    /// Always `Some` at runtime; panics on aarch64.
    #[inline]
    pub fn x86_trace_ref(&self) -> &X86TraceSnapshot {
        let ArchThreadState::X86 { x86_trace, .. } = self else {
            unreachable!("x86_trace_ref called on aarch64 thread");
        };
        x86_trace
            .as_ref()
            .expect("x86 trace snapshot was set at construction")
    }

    /// Return a mutable reference to the aarch64 trace snapshot.
    #[inline]
    pub fn aarch64_trace_mut(&mut self) -> &mut AArch64TraceSnapshot {
        let ArchThreadState::AArch64 { aarch64_trace, .. } = self else {
            unreachable!("aarch64_trace_mut called on x86 thread");
        };
        aarch64_trace
            .as_mut()
            .expect("aarch64 trace snapshot was set at construction")
    }

    /// Return an immutable reference to the aarch64 trace snapshot.
    #[inline]
    pub fn aarch64_trace_ref(&self) -> &AArch64TraceSnapshot {
        let ArchThreadState::AArch64 { aarch64_trace, .. } = self else {
            unreachable!("aarch64_trace_ref called on x86 thread");
        };
        aarch64_trace
            .as_ref()
            .expect("aarch64 trace snapshot was set at construction")
    }
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
                aarch64_trace: Some(Box::new(AArch64TraceSnapshot::new())),
            }
        } else {
            ArchThreadState::X86 {
                regs: Regs64::new(),
                flags: Flags::new(),
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
                x86_trace: Some(Box::new(X86TraceSnapshot::new())),
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
            _ => unreachable!("x86() called on aarch64 thread"),
        }
    }

    #[inline]
    pub fn regs_x86(&self) -> &Regs64 {
        match &self.arch {
            ArchThreadState::X86 { regs, .. } => regs,
            _ => unreachable!("regs_x86 called on aarch64 thread"),
        }
    }

    #[inline]
    pub fn regs_x86_mut(&mut self) -> &mut Regs64 {
        match &mut self.arch {
            ArchThreadState::X86 { regs, .. } => regs,
            _ => unreachable!("regs_x86_mut called on x86 thread"),
        }
    }

    #[inline]
    pub fn regs_aarch64(&self) -> &RegsAarch64 {
        match &self.arch {
            ArchThreadState::AArch64 { regs, .. } => regs,
            _ => unreachable!("regs_aarch64 called on x86 thread"),
        }
    }

    #[inline]
    pub fn regs_aarch64_mut(&mut self) -> &mut RegsAarch64 {
        match &mut self.arch {
            ArchThreadState::AArch64 { regs, .. } => regs,
            _ => unreachable!("regs_aarch64_mut called on x86 thread"),
        }
    }
}
