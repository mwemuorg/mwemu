//! Synchronisation, DPCs, timers and IRQL: the `Ke*` / `Ex*` surface.
//!
//! The emulation is single-threaded, so locks never contend: acquiring one
//! returns immediately and an IRQL change is a value the driver reads back.
//! Events and semaphores are modelled for real because drivers branch on
//! their state. Deferred procedure calls, timers and system threads are
//! *queued*, not run: "unregister now, free later" is the shape of most
//! kernel use-after-free bugs, and running the callback immediately would
//! close the very window the bug lives in. Drain them with
//! `Emu::kernel_run_deferred()`.

use super::structs::*;
use super::*;
use crate::kernel::DeferredCall;

fn dpc_routine(emu: &Emu, dpc: u64) -> (u64, u64) {
    (
        emu.maps
            .read_qword(dpc + KDPC_DEFERRED_ROUTINE)
            .unwrap_or(0),
        emu.maps
            .read_qword(dpc + KDPC_DEFERRED_CONTEXT)
            .unwrap_or(0),
    )
}

fn queue_dpc(emu: &mut Emu, kind: &str, dpc: u64, a1: u64, a2: u64) {
    if dpc == 0 {
        return;
    }
    let (routine, ctx) = dpc_routine(emu, dpc);
    emu.kernel_defer(DeferredCall {
        kind: kind.to_string(),
        func: routine,
        args: vec![dpc, ctx, a1, a2],
    });
}

pub fn dispatch(symbol: &str, emu: &mut Emu) -> bool {
    match symbol {
        // --- spin locks and IRQL -------------------------------------------------
        "KeInitializeSpinLock" | "KeInitializeDeviceQueue" | "KeInitializeQueue" => {
            let p = emu.kernel_arg(0);
            emu.maps.write_qword(p, 0);
            emu.set_kernel_ret(0);
        }
        "KeAcquireSpinLockRaiseToDpc"
        | "KeAcquireSpinLockRaiseToSynch"
        | "KfAcquireSpinLock"
        | "KeAcquireSpinLockForDpc"
        | "ExAcquireSpinLockExclusive"
        | "ExAcquireSpinLockShared"
        | "KeRaiseIrqlToDpcLevel"
        | "KeRaiseIrqlToSynchLevel"
        | "KfRaiseIrql"
        | "KeRaiseIrql"
        | "KeAcquireInterruptSpinLock"
        | "KeGetCurrentIrql"
        | "KeAcquireQueuedSpinLock" => {
            // Returns the previous IRQL: PASSIVE_LEVEL.
            if symbol == "KeRaiseIrql" {
                let out = emu.kernel_arg(1);
                if out != 0 {
                    emu.maps.write_byte(out, 0);
                }
            }
            emu.set_kernel_ret(0);
        }
        "KeAcquireSpinLock" => {
            // Macro on x64; exported for x86. (Lock, *OldIrql)
            let out = emu.kernel_arg(1);
            if out != 0 {
                emu.maps.write_byte(out, 0);
            }
            emu.set_kernel_ret(0);
        }
        "KeReleaseSpinLock"
        | "KfReleaseSpinLock"
        | "KeReleaseSpinLockForDpc"
        | "KeAcquireSpinLockAtDpcLevel"
        | "KefAcquireSpinLockAtDpcLevel"
        | "KeReleaseSpinLockFromDpcLevel"
        | "KefReleaseSpinLockFromDpcLevel"
        | "ExReleaseSpinLockExclusive"
        | "ExReleaseSpinLockShared"
        | "ExAcquireSpinLockExclusiveAtDpcLevel"
        | "ExAcquireSpinLockSharedAtDpcLevel"
        | "ExReleaseSpinLockExclusiveFromDpcLevel"
        | "ExReleaseSpinLockSharedFromDpcLevel"
        | "ExTryConvertSharedSpinLockExclusive"
        | "KeLowerIrql"
        | "KfLowerIrql"
        | "KeReleaseInterruptSpinLock"
        | "KeReleaseQueuedSpinLock"
        | "KeAcquireInStackQueuedSpinLock"
        | "KeAcquireInStackQueuedSpinLockAtDpcLevel"
        | "KeAcquireInStackQueuedSpinLockForDpc"
        | "KeReleaseInStackQueuedSpinLock"
        | "KeReleaseInStackQueuedSpinLockFromDpcLevel"
        | "KeReleaseInStackQueuedSpinLockForDpc"
        | "KeEnterCriticalRegion"
        | "KeLeaveCriticalRegion"
        | "KeEnterGuardedRegion"
        | "KeLeaveGuardedRegion"
        | "KeInitializeGuardedMutex"
        | "KeAcquireGuardedMutex"
        | "KeReleaseGuardedMutex"
        | "KeAcquireGuardedMutexUnsafe"
        | "KeReleaseGuardedMutexUnsafe"
        | "KeFlushQueuedDpcs"
        | "KeSetImportanceDpc"
        | "KeSetTargetProcessorDpc"
        | "KeSetTargetProcessorDpcEx"
        | "KeMemoryBarrier"
        | "KeInvalidateAllCaches"
        | "KeFlushIoBuffers"
        | "KeSweepIcache"
        | "KeSaveFloatingPointState"
        | "KeRestoreFloatingPointState"
        | "KeSaveExtendedProcessorState"
        | "KeRestoreExtendedProcessorState"
        | "KeRevertToUserAffinityThread"
        | "KeRevertToUserAffinityThreadEx"
        | "KeRevertToUserGroupAffinityThread"
        | "KeSetSystemGroupAffinityThread"
        | "KeStackAttachProcess"
        | "KeUnstackDetachProcess"
        | "KeAttachProcess"
        | "KeDetachProcess"
        | "KeSetBasePriorityThread"
        | "KeSetPriorityThread"
        | "KeInitializeApc"
        | "KeInitializeAffinityEx"
        | "KeAddProcessorAffinityEx"
        | "KeRemoveQueueApc"
        | "KeTerminateThread"
        | "KeRegisterBugCheckReasonCallback"
        | "KeDeregisterBugCheckReasonCallback"
        | "KeDeregisterBugCheckCallback"
        | "KeDeregisterNmiCallback"
        | "KeInitializeCrashDumpHeader"
        | "KeSetKernelStackSwapEnable"
        | "KeAbandonMutex"
        | "KeRundownQueue" => {
            emu.set_kernel_ret(0);
        }
        "KeTryToAcquireSpinLockAtDpcLevel"
        | "KeTryToAcquireGuardedMutex"
        | "KeTestSpinLock"
        | "ExTryToAcquireFastMutex"
        | "KeAreApcsDisabled"
        | "KeAreAllApcsDisabled"
        | "KeIsExecutingDpc"
        | "KeIsAttachedProcess"
        | "ExIsResourceAcquiredExclusiveLite"
        | "ExIsResourceAcquiredSharedLite"
        | "KeTryToAcquireQueuedSpinLock" => {
            let v = matches!(
                symbol,
                "KeTryToAcquireSpinLockAtDpcLevel"
                    | "KeTryToAcquireGuardedMutex"
                    | "ExTryToAcquireFastMutex"
                    | "ExIsResourceAcquiredExclusiveLite"
                    | "KeTryToAcquireQueuedSpinLock"
            ) as u64;
            if symbol == "ExTryToAcquireFastMutex" {
                let m = emu.kernel_arg(0);
                emu.maps.write_dword(m + FAST_MUTEX_COUNT, 0);
            }
            emu.set_kernel_ret(v);
        }
        "ExAcquireFastMutex" | "ExAcquireFastMutexUnsafe" => {
            let m = emu.kernel_arg(0);
            let thread = emu
                .kernel
                .as_ref()
                .map(|k| k.win.current_thread)
                .unwrap_or(0);
            emu.maps.write_dword(m + FAST_MUTEX_COUNT, 0);
            emu.maps.write_qword(m + FAST_MUTEX_OWNER, thread);
            emu.set_kernel_ret(0);
        }
        "ExReleaseFastMutex" | "ExReleaseFastMutexUnsafe" => {
            let m = emu.kernel_arg(0);
            emu.maps.write_dword(m + FAST_MUTEX_COUNT, 1);
            emu.maps.write_qword(m + FAST_MUTEX_OWNER, 0);
            emu.set_kernel_ret(0);
        }
        // ERESOURCE, push locks, rundown protection: uncontended by construction.
        "ExInitializeResourceLite"
        | "ExReinitializeResourceLite"
        | "ExDeleteResourceLite"
        | "ExReleaseResourceLite"
        | "ExReleaseResourceForThreadLite"
        | "ExConvertExclusiveToSharedLite"
        | "ExSetResourceOwnerPointer"
        | "ExSetResourceOwnerPointerEx"
        | "ExEnterCriticalRegionAndAcquireResourceExclusive"
        | "ExEnterCriticalRegionAndAcquireResourceShared"
        | "ExReleaseResourceAndLeaveCriticalRegion"
        | "ExInitializePushLock"
        | "ExfAcquirePushLockExclusive"
        | "ExfAcquirePushLockShared"
        | "ExfReleasePushLock"
        | "ExfReleasePushLockExclusive"
        | "ExfReleasePushLockShared"
        | "ExAcquirePushLockExclusiveEx"
        | "ExAcquirePushLockSharedEx"
        | "ExReleasePushLockExclusiveEx"
        | "ExReleasePushLockSharedEx"
        | "ExReleasePushLockEx"
        | "ExfTryToWakePushLock"
        | "ExInitializeRundownProtection"
        | "ExReInitializeRundownProtection"
        | "ExRundownCompleted"
        | "ExReleaseRundownProtection"
        | "ExReleaseRundownProtectionEx"
        | "ExWaitForRundownProtectionRelease"
        | "ExInitializeRundownProtectionCacheAware"
        | "ExReleaseRundownProtectionCacheAware"
        | "ExWaitForRundownProtectionReleaseCacheAware"
        | "ExRundownCompletedCacheAware"
        | "ExReleaseRundownProtectionCacheAwareEx"
        | "ExFreeCacheAwareRundownProtection" => {
            if symbol == "ExInitializeRundownProtection"
                || symbol == "ExReInitializeRundownProtection"
            {
                let p = emu.kernel_arg(0);
                emu.maps.write_qword(p, 0);
            }
            emu.set_kernel_ret(0);
        }
        "ExAcquireResourceExclusiveLite"
        | "ExAcquireResourceSharedLite"
        | "ExAcquireSharedStarveExclusive"
        | "ExAcquireSharedWaitForExclusive"
        | "ExAcquireRundownProtection"
        | "ExAcquireRundownProtectionEx"
        | "ExAcquireRundownProtectionCacheAware"
        | "ExAcquireRundownProtectionCacheAwareEx"
        | "ExTryAcquirePushLockExclusiveEx"
        | "ExTryAcquirePushLockSharedEx" => {
            emu.set_kernel_ret(1);
        }
        "ExGetExclusiveWaiterCount" | "ExGetSharedWaiterCount" => emu.set_kernel_ret(0),
        "ExAllocateCacheAwareRundownProtection" => {
            let p = emu.kernel_alloc(
                crate::kernel::heap::Region::Slab,
                0x40,
                "rundown",
                symbol,
                true,
            );
            emu.set_kernel_ret(p);
        }

        // --- events, semaphores, mutexes ---------------------------------------------
        "KeInitializeEvent" => {
            let ev = emu.kernel_arg(0);
            let t = emu.kernel_arg(1) as u8;
            let state = emu.kernel_arg(2) as u32;
            emu.maps.write_bytes(ev, &[0u8; KEVENT_SIZE as usize]);
            emu.maps.write_byte(ev + DISPATCHER_TYPE, t);
            emu.maps.write_dword(ev + DISPATCHER_SIGNAL_STATE, state);
            emu.maps.write_qword(ev + 8, ev + 8);
            emu.maps.write_qword(ev + 0x10, ev + 8);
            emu.set_kernel_ret(0);
        }
        "KeSetEvent" | "KeSetEventBoostPriority" => {
            let ev = emu.kernel_arg(0);
            let old = emu
                .maps
                .read_dword(ev + DISPATCHER_SIGNAL_STATE)
                .unwrap_or(0);
            emu.maps.write_dword(ev + DISPATCHER_SIGNAL_STATE, 1);
            emu.set_kernel_ret(old as u64);
        }
        "KeResetEvent" | "KeClearEvent" => {
            let ev = emu.kernel_arg(0);
            let old = emu
                .maps
                .read_dword(ev + DISPATCHER_SIGNAL_STATE)
                .unwrap_or(0);
            emu.maps.write_dword(ev + DISPATCHER_SIGNAL_STATE, 0);
            emu.set_kernel_ret(old as u64);
        }
        "KeReadStateEvent"
        | "KeReadStateMutex"
        | "KeReadStateSemaphore"
        | "KeReadStateTimer"
        | "KeReadStateMutant" => {
            let obj = emu.kernel_arg(0);
            let v = emu
                .maps
                .read_dword(obj + DISPATCHER_SIGNAL_STATE)
                .unwrap_or(0);
            emu.set_kernel_ret(v as u64);
        }
        "KeInitializeMutex" | "KeInitializeMutant" => {
            let m = emu.kernel_arg(0);
            emu.maps.write_bytes(m, &[0u8; 0x38]);
            emu.maps.write_byte(m + DISPATCHER_TYPE, MUTANT_OBJECT);
            emu.maps.write_dword(m + DISPATCHER_SIGNAL_STATE, 1);
            emu.set_kernel_ret(0);
        }
        "KeReleaseMutex" | "KeReleaseMutant" => {
            let m = emu.kernel_arg(0);
            let old = emu
                .maps
                .read_dword(m + DISPATCHER_SIGNAL_STATE)
                .unwrap_or(0);
            emu.maps.write_dword(m + DISPATCHER_SIGNAL_STATE, 1);
            emu.set_kernel_ret(old as u64);
        }
        "KeInitializeSemaphore" => {
            let s = emu.kernel_arg(0);
            let count = emu.kernel_arg(1) as u32;
            let limit = emu.kernel_arg(2) as u32;
            emu.maps.write_bytes(s, &[0u8; 0x20]);
            emu.maps.write_byte(s + DISPATCHER_TYPE, SEMAPHORE_OBJECT);
            emu.maps.write_dword(s + DISPATCHER_SIGNAL_STATE, count);
            emu.maps.write_dword(s + 0x18, limit);
            emu.set_kernel_ret(0);
        }
        "KeReleaseSemaphore" => {
            let s = emu.kernel_arg(0);
            let adj = emu.kernel_arg(2) as u32;
            let old = emu
                .maps
                .read_dword(s + DISPATCHER_SIGNAL_STATE)
                .unwrap_or(0);
            emu.maps
                .write_dword(s + DISPATCHER_SIGNAL_STATE, old.wrapping_add(adj));
            emu.set_kernel_ret(old as u64);
        }
        // KeWaitForSingleObject(Object, Reason, Mode, Alertable, *Timeout)
        "KeWaitForSingleObject" | "KeWaitForMutexObject" => {
            let obj = emu.kernel_arg(0);
            let timeout = emu.kernel_arg(4);
            let t = emu.maps.read_byte(obj + DISPATCHER_TYPE).unwrap_or(0);
            let state = emu
                .maps
                .read_dword(obj + DISPATCHER_SIGNAL_STATE)
                .unwrap_or(0);
            let signaled = state != 0 || t == MUTANT_OBJECT && state == 1;
            // A synchronisation event, mutex or semaphore is consumed by the wait.
            if signaled {
                match t {
                    SYNCHRONIZATION_EVENT | SYNCHRONIZATION_TIMER => {
                        emu.maps.write_dword(obj + DISPATCHER_SIGNAL_STATE, 0);
                    }
                    SEMAPHORE_OBJECT | MUTANT_OBJECT => {
                        emu.maps
                            .write_dword(obj + DISPATCHER_SIGNAL_STATE, state.wrapping_sub(1));
                    }
                    _ => {}
                }
                emu.set_kernel_ret(STATUS_SUCCESS);
            } else if timeout != 0 {
                // Nothing else runs, so the wait can only time out.
                emu.set_kernel_ret(STATUS_TIMEOUT);
            } else {
                // An infinite wait on an unsignaled object would hang a real
                // kernel; here the pending work is what would signal it, so
                // treat the wait as satisfied and let the driver continue.
                log::info!("{}: object 0x{:x} not signaled, continuing", symbol, obj);
                emu.set_kernel_ret(STATUS_SUCCESS);
            }
        }
        "KeWaitForMultipleObjects" => emu.set_kernel_ret(STATUS_SUCCESS),
        "KeDelayExecutionThread" | "KeYieldProcessorEx" => emu.set_kernel_ret(STATUS_SUCCESS),
        "KePulseEvent" => {
            let ev = emu.kernel_arg(0);
            let old = emu
                .maps
                .read_dword(ev + DISPATCHER_SIGNAL_STATE)
                .unwrap_or(0);
            emu.set_kernel_ret(old as u64);
        }

        // --- DPCs and timers -------------------------------------------------------------
        "KeInitializeDpc" | "KeInitializeThreadedDpc" => {
            let dpc = emu.kernel_arg(0);
            let routine = emu.kernel_arg(1);
            let ctx = emu.kernel_arg(2);
            emu.maps.write_bytes(dpc, &[0u8; KDPC_SIZE as usize]);
            emu.maps.write_byte(dpc + KDPC_TYPE, KDPC_TYPE_VALUE);
            emu.maps.write_qword(dpc + KDPC_DEFERRED_ROUTINE, routine);
            emu.maps.write_qword(dpc + KDPC_DEFERRED_CONTEXT, ctx);
            emu.set_kernel_ret(0);
        }
        "KeInsertQueueDpc" => {
            let dpc = emu.kernel_arg(0);
            let a1 = emu.kernel_arg(1);
            let a2 = emu.kernel_arg(2);
            queue_dpc(emu, "dpc", dpc, a1, a2);
            emu.set_kernel_ret(1);
        }
        "KeRemoveQueueDpc" => {
            let dpc = emu.kernel_arg(0);
            let (routine, _) = dpc_routine(emu, dpc);
            let removed = emu
                .kernel
                .as_mut()
                .map(|k| {
                    let before = k.deferred.len();
                    k.deferred
                        .retain(|d| !(d.kind == "dpc" || d.kind == "timer") || d.func != routine);
                    before != k.deferred.len()
                })
                .unwrap_or(false);
            emu.set_kernel_ret(removed as u64);
        }
        "KeInitializeTimer" | "KeInitializeTimerEx" => {
            let t = emu.kernel_arg(0);
            let kind = if symbol == "KeInitializeTimerEx" {
                emu.kernel_arg(1) as u8
            } else {
                0
            };
            emu.maps.write_bytes(t, &[0u8; 0x40]);
            emu.maps
                .write_byte(t + DISPATCHER_TYPE, NOTIFICATION_TIMER + kind);
            emu.maps.write_qword(t + 8, t + 8);
            emu.maps.write_qword(t + 0x10, t + 8);
            emu.set_kernel_ret(0);
        }
        // KeSetTimer(Timer, DueTime, Dpc) / KeSetTimerEx(Timer, DueTime, Period, Dpc)
        "KeSetTimer" | "KeSetTimerEx" | "KeSetCoalescableTimer" => {
            let t = emu.kernel_arg(0);
            let dpc = if symbol == "KeSetTimer" {
                emu.kernel_arg(2)
            } else {
                emu.kernel_arg(3)
            };
            let period = if symbol == "KeSetTimer" {
                0
            } else {
                emu.kernel_arg(2) as u32
            };
            let was_set = emu.maps.read_qword(t + KTIMER_DPC).unwrap_or(0) != 0;
            emu.maps.write_qword(t + KTIMER_DPC, dpc);
            emu.maps.write_dword(t + KTIMER_PERIOD, period);
            // The timer "fires": its DPC is queued once. A periodic timer is
            // not re-armed, which keeps a driver with a heartbeat from
            // queueing forever.
            emu.maps.write_dword(t + DISPATCHER_SIGNAL_STATE, 1);
            queue_dpc(emu, "timer", dpc, 0, 0);
            emu.set_kernel_ret(was_set as u64);
        }
        "KeCancelTimer" => {
            let t = emu.kernel_arg(0);
            let dpc = emu.maps.read_qword(t + KTIMER_DPC).unwrap_or(0);
            let (routine, _) = dpc_routine(emu, dpc);
            let was_set = dpc != 0;
            emu.maps.write_qword(t + KTIMER_DPC, 0);
            if let Some(k) = emu.kernel.as_mut() {
                k.deferred
                    .retain(|d| d.kind != "timer" || d.func != routine);
            }
            emu.set_kernel_ret(was_set as u64);
        }
        "KeInsertQueueApc" => {
            // KeInsertQueueApc(Apc, SystemArgument1, SystemArgument2, Increment):
            // the kernel routine was stored by KeInitializeApc at +0x30
            // (KernelRoutine) with NormalRoutine at +0x40, NormalContext +0x48.
            let apc = emu.kernel_arg(0);
            let kernel_routine = emu.maps.read_qword(apc + 0x30).unwrap_or(0);
            let normal = emu.maps.read_qword(apc + 0x40).unwrap_or(0);
            let ctx = emu.maps.read_qword(apc + 0x48).unwrap_or(0);
            let a1 = emu.kernel_arg(1);
            let a2 = emu.kernel_arg(2);
            if kernel_routine != 0 {
                emu.kernel_defer(DeferredCall {
                    kind: "apc".to_string(),
                    func: kernel_routine,
                    args: vec![apc, normal, ctx, a1, a2],
                });
            }
            emu.set_kernel_ret(1);
        }
        "KeRegisterNmiCallback"
        | "KeRegisterBugCheckCallback"
        | "KeRegisterProcessorChangeCallback" => {
            let func = emu.kernel_arg(0);
            let ctx = emu.kernel_arg(1);
            if let Some(k) = emu.kernel.as_mut() {
                k.win.callbacks.push(RegisteredCallback {
                    api: symbol.to_string(),
                    func,
                    context: ctx,
                });
            }
            let h = new_handle(emu, symbol.to_string());
            emu.set_kernel_ret(if symbol == "KeRegisterBugCheckCallback" {
                1
            } else {
                h
            });
        }
        // KeIpiGenericCall(BroadcastFunction, Context): runs the function on
        // every processor and returns its result — a tail call here.
        "KeIpiGenericCall" => {
            let func = emu.kernel_arg(0);
            let ctx = emu.kernel_arg(1);
            emu.kernel_privop("ipi_call", func, ctx, 0);
            emu.kernel_tail_call(func, &[ctx]);
        }
        // KeExpandKernelStackAndCallout(Callout, Parameter, Size) /
        // KeExpandKernelStackAndCalloutEx(Callout, Parameter, Size, Wait, Context)
        "KeExpandKernelStackAndCallout" | "KeExpandKernelStackAndCalloutEx" => {
            let func = emu.kernel_arg(0);
            let param = emu.kernel_arg(1);
            emu.kernel_tail_call(func, &[param]);
        }
        "KeGenericCallDpc" => {
            let routine = emu.kernel_arg(0);
            let ctx = emu.kernel_arg(1);
            emu.kernel_defer(DeferredCall {
                kind: "generic_dpc".to_string(),
                func: routine,
                args: vec![0, ctx, 0, 0],
            });
            emu.set_kernel_ret(0);
        }
        "KeSignalCallDpcDone" | "KeSignalCallDpcSynchronize" => emu.set_kernel_ret(1),

        // --- processors, affinity, time ----------------------------------------------------
        "KeGetCurrentProcessorNumber"
        | "KeGetCurrentProcessorNumberEx"
        | "KeGetCurrentNodeNumber"
        | "KeGetProcessorNumberFromIndex"
        | "KeGetProcessorIndexFromNumber"
        | "KeQueryHighestNodeNumber"
        | "KeQueryNodeMaximumProcessorCount"
        | "KeGetCurrentIrqlEx" => {
            if symbol == "KeGetCurrentProcessorNumberEx" {
                let out = emu.kernel_arg(0);
                if out != 0 {
                    emu.maps.write_dword(out, 0);
                }
            }
            if symbol == "KeGetProcessorNumberFromIndex" {
                let out = emu.kernel_arg(1);
                if out != 0 {
                    emu.maps.write_dword(out, 0);
                }
            }
            emu.set_kernel_ret(0);
        }
        "KeQueryActiveProcessorCount"
        | "KeQueryActiveProcessorCountEx"
        | "KeQueryMaximumProcessorCount"
        | "KeQueryMaximumProcessorCountEx"
        | "KeQueryActiveProcessors"
        | "KeQueryGroupAffinity"
        | "KeQueryActiveGroupCount"
        | "KeQueryMaximumGroupCount"
        | "KeSetSystemAffinityThreadEx"
        | "KeQueryNodeActiveAffinity"
        | "KeQueryLogicalProcessorRelationship" => {
            if symbol == "KeQueryActiveProcessorCount" {
                let out = emu.kernel_arg(0);
                if out != 0 {
                    emu.maps.write_qword(out, 1);
                }
            }
            emu.set_kernel_ret(1);
        }
        "KeSetSystemAffinityThread" => emu.set_kernel_ret(0),
        "KeQueryPerformanceCounter" => {
            let freq = emu.kernel_arg(0);
            if freq != 0 {
                emu.maps.write_qword(freq, 10_000_000);
            }
            emu.set_kernel_ret(emu.pos.wrapping_mul(10));
        }
        "KeQuerySystemTime" | "KeQuerySystemTimePrecise" => {
            let out = emu.kernel_arg(0);
            let t = system_time_now();
            write_out_qword(emu, out, t);
            emu.set_kernel_ret(0);
        }
        "KeQueryInterruptTime"
        | "KeQueryUnbiasedInterruptTime"
        | "KeQueryInterruptTimePrecise"
        | "KeQueryUnbiasedInterruptTimePrecise" => {
            let t = emu.pos.wrapping_mul(100);
            let out = emu.kernel_arg(0);
            if symbol.ends_with("Precise") {
                write_out_qword(emu, out, t);
            }
            emu.set_kernel_ret(t);
        }
        "KeQueryTickCount" => {
            let out = emu.kernel_arg(0);
            write_out_qword(emu, out, emu.pos / 1000);
            emu.set_kernel_ret(0);
        }
        "KeQueryTimeIncrement" => emu.set_kernel_ret(156_250),
        "KeGetCurrentThread" | "PsGetCurrentThread" | "KeGetCurrentThreadEx" => {
            let t = emu
                .kernel
                .as_ref()
                .map(|k| k.win.current_thread)
                .unwrap_or(0);
            emu.set_kernel_ret(t);
        }
        "KeGetRecommendedSharedDataAlignment" => emu.set_kernel_ret(64),
        "KeCapturePersistentThreadState" | "KeSetTimeIncrement" | "KeSetDmaIoCoherency" => {
            emu.set_kernel_ret(0);
        }
        "KeQueryPriorityThread" | "KeQueryRuntimeThread" => emu.set_kernel_ret(8),
        "KeIsProcessorFeaturePresent" | "ExIsProcessorFeaturePresent" => emu.set_kernel_ret(1),

        // --- bugcheck ---------------------------------------------------------------------
        "KeBugCheck" | "KeBugCheckEx" => {
            let code = emu.kernel_arg(0);
            let (a, b, c, d) = if symbol == "KeBugCheckEx" {
                (
                    emu.kernel_arg(1),
                    emu.kernel_arg(2),
                    emu.kernel_arg(3),
                    emu.kernel_arg(4),
                )
            } else {
                (0, 0, 0, 0)
            };
            let line = format!(
                "*** STOP: 0x{:08X} (0x{:x}, 0x{:x}, 0x{:x}, 0x{:x})",
                code, a, b, c, d
            );
            log::error!("{}: {}", symbol, line);
            emu.kernel_log_line(line);
            emu.stop();
        }
        "ExRaiseStatus" | "ExRaiseAccessViolation" | "ExRaiseDatatypeMisalignment" => {
            let status = if symbol == "ExRaiseStatus" {
                emu.kernel_arg(0)
            } else {
                STATUS_ACCESS_VIOLATION
            };
            log::error!(
                "{}: 0x{:x} — no SEH dispatch in kernel mode, stopping",
                symbol,
                status
            );
            emu.kernel_log_line(format!("{}(0x{:x})", symbol, status));
            emu.stop();
        }
        _ => return false,
    }
    true
}

pub const SURFACE: &[&str] = &[
    "KeInitializeSpinLock",
    "KeAcquireSpinLock",
    "KeAcquireSpinLockRaiseToDpc",
    "KeAcquireSpinLockRaiseToSynch",
    "KeReleaseSpinLock",
    "KeAcquireSpinLockAtDpcLevel",
    "KeReleaseSpinLockFromDpcLevel",
    "KeTryToAcquireSpinLockAtDpcLevel",
    "KeAcquireInStackQueuedSpinLock",
    "KeAcquireInStackQueuedSpinLockAtDpcLevel",
    "KeReleaseInStackQueuedSpinLock",
    "KeReleaseInStackQueuedSpinLockFromDpcLevel",
    "KeAcquireInterruptSpinLock",
    "KeReleaseInterruptSpinLock",
    "ExAcquireSpinLockExclusive",
    "ExAcquireSpinLockShared",
    "ExReleaseSpinLockExclusive",
    "ExReleaseSpinLockShared",
    "ExAcquireSpinLockSharedAtDpcLevel",
    "ExReleaseSpinLockSharedFromDpcLevel",
    "KeGetCurrentIrql",
    "KeRaiseIrql",
    "KfRaiseIrql",
    "KeRaiseIrqlToDpcLevel",
    "KeLowerIrql",
    "KfLowerIrql",
    "KeEnterCriticalRegion",
    "KeLeaveCriticalRegion",
    "KeEnterGuardedRegion",
    "KeLeaveGuardedRegion",
    "KeAreApcsDisabled",
    "KeInitializeGuardedMutex",
    "KeAcquireGuardedMutex",
    "KeReleaseGuardedMutex",
    "KeTryToAcquireGuardedMutex",
    "ExAcquireFastMutex",
    "ExReleaseFastMutex",
    "ExTryToAcquireFastMutex",
    "ExAcquireFastMutexUnsafe",
    "ExReleaseFastMutexUnsafe",
    "ExInitializeResourceLite",
    "ExReinitializeResourceLite",
    "ExDeleteResourceLite",
    "ExAcquireResourceExclusiveLite",
    "ExAcquireResourceSharedLite",
    "ExAcquireSharedStarveExclusive",
    "ExAcquireSharedWaitForExclusive",
    "ExReleaseResourceLite",
    "ExReleaseResourceForThreadLite",
    "ExIsResourceAcquiredExclusiveLite",
    "ExIsResourceAcquiredSharedLite",
    "ExGetExclusiveWaiterCount",
    "ExGetSharedWaiterCount",
    "ExInitializePushLock",
    "ExfAcquirePushLockExclusive",
    "ExfAcquirePushLockShared",
    "ExfReleasePushLock",
    "ExfReleasePushLockExclusive",
    "ExfReleasePushLockShared",
    "ExAcquirePushLockExclusiveEx",
    "ExAcquirePushLockSharedEx",
    "ExReleasePushLockExclusiveEx",
    "ExReleasePushLockSharedEx",
    "ExReleasePushLockEx",
    "ExInitializeRundownProtection",
    "ExReInitializeRundownProtection",
    "ExAcquireRundownProtection",
    "ExAcquireRundownProtectionEx",
    "ExReleaseRundownProtection",
    "ExReleaseRundownProtectionEx",
    "ExWaitForRundownProtectionRelease",
    "ExRundownCompleted",
    "KeInitializeEvent",
    "KeSetEvent",
    "KeResetEvent",
    "KeClearEvent",
    "KeReadStateEvent",
    "KePulseEvent",
    "KeInitializeMutex",
    "KeReleaseMutex",
    "KeReadStateMutex",
    "KeInitializeSemaphore",
    "KeReleaseSemaphore",
    "KeReadStateSemaphore",
    "KeWaitForSingleObject",
    "KeWaitForMutexObject",
    "KeWaitForMultipleObjects",
    "KeDelayExecutionThread",
    "KeInitializeDpc",
    "KeInitializeThreadedDpc",
    "KeInsertQueueDpc",
    "KeRemoveQueueDpc",
    "KeSetImportanceDpc",
    "KeSetTargetProcessorDpc",
    "KeSetTargetProcessorDpcEx",
    "KeFlushQueuedDpcs",
    "KeInitializeTimer",
    "KeInitializeTimerEx",
    "KeSetTimer",
    "KeSetTimerEx",
    "KeSetCoalescableTimer",
    "KeCancelTimer",
    "KeReadStateTimer",
    "KeInitializeApc",
    "KeInsertQueueApc",
    "KeRemoveQueueApc",
    "KeRegisterNmiCallback",
    "KeDeregisterNmiCallback",
    "KeRegisterBugCheckCallback",
    "KeDeregisterBugCheckCallback",
    "KeRegisterBugCheckReasonCallback",
    "KeDeregisterBugCheckReasonCallback",
    "KeIpiGenericCall",
    "KeExpandKernelStackAndCallout",
    "KeExpandKernelStackAndCalloutEx",
    "KeGenericCallDpc",
    "KeSignalCallDpcDone",
    "KeSignalCallDpcSynchronize",
    "KeGetCurrentProcessorNumber",
    "KeGetCurrentProcessorNumberEx",
    "KeGetCurrentNodeNumber",
    "KeGetProcessorNumberFromIndex",
    "KeGetProcessorIndexFromNumber",
    "KeQueryActiveProcessorCount",
    "KeQueryActiveProcessorCountEx",
    "KeQueryMaximumProcessorCount",
    "KeQueryMaximumProcessorCountEx",
    "KeQueryActiveProcessors",
    "KeQueryActiveGroupCount",
    "KeQueryMaximumGroupCount",
    "KeQueryGroupAffinity",
    "KeQueryNodeActiveAffinity",
    "KeSetSystemAffinityThread",
    "KeSetSystemAffinityThreadEx",
    "KeRevertToUserAffinityThread",
    "KeRevertToUserAffinityThreadEx",
    "KeSetSystemGroupAffinityThread",
    "KeRevertToUserGroupAffinityThread",
    "KeInitializeAffinityEx",
    "KeAddProcessorAffinityEx",
    "KeStackAttachProcess",
    "KeUnstackDetachProcess",
    "KeAttachProcess",
    "KeDetachProcess",
    "KeSetBasePriorityThread",
    "KeSetPriorityThread",
    "KeQueryPriorityThread",
    "KeQueryRuntimeThread",
    "KeQueryPerformanceCounter",
    "KeQuerySystemTime",
    "KeQuerySystemTimePrecise",
    "KeQueryInterruptTime",
    "KeQueryUnbiasedInterruptTime",
    "KeQueryInterruptTimePrecise",
    "KeQueryUnbiasedInterruptTimePrecise",
    "KeQueryTickCount",
    "KeQueryTimeIncrement",
    "KeGetCurrentThread",
    "PsGetCurrentThread",
    "KeGetRecommendedSharedDataAlignment",
    "KeCapturePersistentThreadState",
    "KeIsProcessorFeaturePresent",
    "ExIsProcessorFeaturePresent",
    "KeIsExecutingDpc",
    "KeMemoryBarrier",
    "KeInvalidateAllCaches",
    "KeFlushIoBuffers",
    "KeSaveFloatingPointState",
    "KeRestoreFloatingPointState",
    "KeSaveExtendedProcessorState",
    "KeRestoreExtendedProcessorState",
    "KeBugCheck",
    "KeBugCheckEx",
    "ExRaiseStatus",
    "ExRaiseAccessViolation",
    "ExRaiseDatatypeMisalignment",
];
