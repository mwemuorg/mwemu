use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::eflags::Eflags;
use crate::flags::Flags;
use crate::regs_aarch64::RegsAarch64;
use crate::regs64::Regs64;
use crate::serialization::fpu::SerializableFPU;
use crate::threading::context::{ArchThreadState, ThreadContext};

#[derive(Serialize, Deserialize)]
pub enum SerializableThreadArch {
    X86 {
        regs: Regs64,
        pre_op_regs: Regs64,
        post_op_regs: Regs64,
        flags: Flags,
        pre_op_flags: Flags,
        post_op_flags: Flags,
        eflags: Eflags,
        fpu: SerializableFPU,
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

#[derive(Serialize, Deserialize)]
pub struct SerializableThreadContext {
    pub id: u64,
    pub suspended: bool,
    pub wake_tick: usize,
    pub blocked_on_cs: Option<u64>,
    pub handle: u64,
    pub arch: SerializableThreadArch,
}

impl From<&ThreadContext> for SerializableThreadContext {
    fn from(thread: &ThreadContext) -> Self {
        match &thread.arch {
            ArchThreadState::X86 {
                regs,
                pre_op_regs,
                post_op_regs,
                flags,
                pre_op_flags,
                post_op_flags,
                eflags,
                fpu,
                seh,
                veh,
                uef,
                eh_ctx,
                tls32,
                tls64,
                fls,
                fs,
                call_stack,
            } => SerializableThreadContext {
                id: thread.id,
                suspended: thread.suspended,
                wake_tick: thread.wake_tick,
                blocked_on_cs: thread.blocked_on_cs,
                handle: thread.handle,
                arch: SerializableThreadArch::X86 {
                    regs: *regs,
                    pre_op_regs: *pre_op_regs,
                    post_op_regs: *post_op_regs,
                    flags: *flags,
                    pre_op_flags: *pre_op_flags,
                    post_op_flags: *post_op_flags,
                    eflags: eflags.clone(),
                    fpu: fpu.clone().into(),
                    seh: *seh,
                    veh: *veh,
                    uef: *uef,
                    eh_ctx: *eh_ctx,
                    tls32: tls32.clone(),
                    tls64: tls64.clone(),
                    fls: fls.clone(),
                    fs: fs.clone(),
                    call_stack: call_stack.clone(),
                },
            },
            ArchThreadState::AArch64 {
                regs,
                pre_op_regs,
                post_op_regs,
            } => SerializableThreadContext {
                id: thread.id,
                suspended: thread.suspended,
                wake_tick: thread.wake_tick,
                blocked_on_cs: thread.blocked_on_cs,
                handle: thread.handle,
                arch: SerializableThreadArch::AArch64 {
                    regs: *regs,
                    pre_op_regs: *pre_op_regs,
                    post_op_regs: *post_op_regs,
                },
            },
        }
    }
}

impl From<SerializableThreadContext> for ThreadContext {
    fn from(serialized: SerializableThreadContext) -> Self {
        ThreadContext {
            id: serialized.id,
            suspended: serialized.suspended,
            wake_tick: serialized.wake_tick,
            blocked_on_cs: serialized.blocked_on_cs,
            handle: serialized.handle,
            arch: match serialized.arch {
                SerializableThreadArch::X86 {
                    regs,
                    pre_op_regs,
                    post_op_regs,
                    flags,
                    pre_op_flags,
                    post_op_flags,
                    eflags,
                    fpu,
                    seh,
                    veh,
                    uef,
                    eh_ctx,
                    tls32,
                    tls64,
                    fls,
                    fs,
                    call_stack,
                } => ArchThreadState::X86 {
                    regs,
                    pre_op_regs,
                    post_op_regs,
                    flags,
                    pre_op_flags,
                    post_op_flags,
                    eflags,
                    fpu: fpu.into(),
                    seh,
                    veh,
                    uef,
                    eh_ctx,
                    tls32,
                    tls64,
                    fls,
                    fs,
                    call_stack,
                },
                SerializableThreadArch::AArch64 {
                    regs,
                    pre_op_regs,
                    post_op_regs,
                } => ArchThreadState::AArch64 {
                    regs,
                    pre_op_regs,
                    post_op_regs,
                },
            },
        }
    }
}
