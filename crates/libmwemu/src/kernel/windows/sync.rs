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

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::api::windows::policy::api_try;
use crate::kernel::{HandlerRegistry, kernel_handlers, return_one, return_zero};

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
    dispatch_api!(symbol, emu, registry handlers(),
        handled(()) => true,
        fallback => false,
    )
}

pub const fn handlers() -> HandlerRegistry {
    kernel_handlers! {
        // --- spin locks and IRQL -------------------------------------------------
        "KeInitializeSpinLock" | "KeInitializeDeviceQueue" | "KeInitializeQueue" => synchronization_initialize_stub,
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
        | "KeAcquireQueuedSpinLock" => irql_stub,
        "KeAcquireSpinLock" => KeAcquireSpinLock,
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
        | "KeRundownQueue" => return_zero,
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
        | "KeTryToAcquireQueuedSpinLock" => synchronization_query_stub,
        "ExAcquireFastMutex" | "ExAcquireFastMutexUnsafe" => ExAcquireFastMutex,
        "ExReleaseFastMutex" | "ExReleaseFastMutexUnsafe" => ExReleaseFastMutex,
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
        | "ExFreeCacheAwareRundownProtection" => resource_rundown_stub,
        "ExAcquireResourceExclusiveLite"
        | "ExAcquireResourceSharedLite"
        | "ExAcquireSharedStarveExclusive"
        | "ExAcquireSharedWaitForExclusive"
        | "ExAcquireRundownProtection"
        | "ExAcquireRundownProtectionEx"
        | "ExAcquireRundownProtectionCacheAware"
        | "ExAcquireRundownProtectionCacheAwareEx"
        | "ExTryAcquirePushLockExclusiveEx"
        | "ExTryAcquirePushLockSharedEx" => return_one,
        "ExGetExclusiveWaiterCount" | "ExGetSharedWaiterCount" => return_zero,
        "ExAllocateCacheAwareRundownProtection" => ExAllocateCacheAwareRundownProtection,

        // --- events, semaphores, mutexes ---------------------------------------------
        "KeInitializeEvent" => KeInitializeEvent,
        "KeSetEvent" | "KeSetEventBoostPriority" => KeSetEvent,
        "KeResetEvent" | "KeClearEvent" => KeResetEvent,
        "KeReadStateEvent"
        | "KeReadStateMutex"
        | "KeReadStateSemaphore"
        | "KeReadStateTimer"
        | "KeReadStateMutant" => KeReadStateEvent,
        "KeInitializeMutex" | "KeInitializeMutant" => KeInitializeMutex,
        "KeReleaseMutex" | "KeReleaseMutant" => KeReleaseMutex,
        "KeInitializeSemaphore" => KeInitializeSemaphore,
        "KeReleaseSemaphore" => KeReleaseSemaphore,
        // KeWaitForSingleObject(Object, Reason, Mode, Alertable, *Timeout)
        "KeWaitForSingleObject" | "KeWaitForMutexObject" => KeWaitForSingleObject,
        "KeWaitForMultipleObjects" => wait_stub,
        "KeDelayExecutionThread" | "KeYieldProcessorEx" => wait_stub,
        "KePulseEvent" => KePulseEvent,

        // --- DPCs and timers -------------------------------------------------------------
        "KeInitializeDpc" | "KeInitializeThreadedDpc" => KeInitializeDpc,
        "KeInsertQueueDpc" => KeInsertQueueDpc,
        "KeRemoveQueueDpc" => KeRemoveQueueDpc,
        "KeInitializeTimer" | "KeInitializeTimerEx" => KeInitializeTimer,
        // KeSetTimer(Timer, DueTime, Dpc) / KeSetTimerEx(Timer, DueTime, Period, Dpc)
        "KeSetTimer" | "KeSetTimerEx" | "KeSetCoalescableTimer" => KeSetTimer,
        "KeCancelTimer" => KeCancelTimer,
        "KeInsertQueueApc" => KeInsertQueueApc,
        "KeRegisterNmiCallback"
        | "KeRegisterBugCheckCallback"
        | "KeRegisterProcessorChangeCallback" => KeRegisterNmiCallback,
        // KeIpiGenericCall(BroadcastFunction, Context): runs the function on
        // every processor and returns its result — a tail call here.
        "KeIpiGenericCall" => KeIpiGenericCall,
        // KeExpandKernelStackAndCallout(Callout, Parameter, Size) /
        // KeExpandKernelStackAndCalloutEx(Callout, Parameter, Size, Wait, Context)
        "KeExpandKernelStackAndCallout" | "KeExpandKernelStackAndCalloutEx" => KeExpandKernelStackAndCallout,
        "KeGenericCallDpc" => KeGenericCallDpc,
        "KeSignalCallDpcDone" | "KeSignalCallDpcSynchronize" => return_one,

        // --- processors, affinity, time ----------------------------------------------------
        "KeGetCurrentProcessorNumber"
        | "KeGetCurrentProcessorNumberEx"
        | "KeGetCurrentNodeNumber"
        | "KeGetProcessorNumberFromIndex"
        | "KeGetProcessorIndexFromNumber"
        | "KeQueryHighestNodeNumber"
        | "KeQueryNodeMaximumProcessorCount"
        | "KeGetCurrentIrqlEx" => processor_number_stub,
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
        | "KeQueryLogicalProcessorRelationship" => processor_affinity_stub,
        "KeSetSystemAffinityThread" => return_zero,
        "KeQueryPerformanceCounter" => KeQueryPerformanceCounter,
        "KeQuerySystemTime" | "KeQuerySystemTimePrecise" => KeQuerySystemTime,
        "KeQueryInterruptTime"
        | "KeQueryUnbiasedInterruptTime"
        | "KeQueryInterruptTimePrecise"
        | "KeQueryUnbiasedInterruptTimePrecise" => KeQueryInterruptTime,
        "KeQueryTickCount" => KeQueryTickCount,
        "KeQueryTimeIncrement" => KeQueryTimeIncrement,
        "KeGetCurrentThread" | "PsGetCurrentThread" | "KeGetCurrentThreadEx" => KeGetCurrentThread,
        "KeGetRecommendedSharedDataAlignment" => KeGetRecommendedSharedDataAlignment,
        "KeCapturePersistentThreadState" | "KeSetTimeIncrement" | "KeSetDmaIoCoherency" => return_zero,
        "KeQueryPriorityThread" | "KeQueryRuntimeThread" => thread_query_stub,
        "KeIsProcessorFeaturePresent" | "ExIsProcessorFeaturePresent" => return_one,

        // --- bugcheck ---------------------------------------------------------------------
        "KeBugCheck" | "KeBugCheckEx" => KeBugCheck,
        "ExRaiseStatus" | "ExRaiseAccessViolation" | "ExRaiseDatatypeMisalignment" => ExRaiseStatus,
    }
}

pub const SURFACE: &[&str] = handlers().names;

pub fn has_handler(symbol: &str) -> bool {
    handlers().find(symbol).is_some()
}

fn synchronization_initialize_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let p = api_try!(emu, call.arg(emu, 0));
    emu.maps.write_qword(p, 0);
    call.set_return(emu, 0);
}

fn irql_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    // Returns the previous IRQL: PASSIVE_LEVEL.
    if symbol == "KeRaiseIrql" {
        let out = api_try!(emu, call.arg(emu, 1));
        if out != 0 {
            emu.maps.write_byte(out, 0);
        }
    }
    call.set_return(emu, 0);
}

fn KeAcquireSpinLock(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    // Macro on x64; exported for x86. (Lock, *OldIrql)
    let out = api_try!(emu, call.arg(emu, 1));
    if out != 0 {
        emu.maps.write_byte(out, 0);
    }
    call.set_return(emu, 0);
}

fn synchronization_query_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let v = matches!(
        symbol,
        "KeTryToAcquireSpinLockAtDpcLevel"
            | "KeTryToAcquireGuardedMutex"
            | "ExTryToAcquireFastMutex"
            | "ExIsResourceAcquiredExclusiveLite"
            | "KeTryToAcquireQueuedSpinLock"
    ) as u64;
    if symbol == "ExTryToAcquireFastMutex" {
        let m = api_try!(emu, call.arg(emu, 0));
        emu.maps.write_dword(m + FAST_MUTEX_COUNT, 0);
    }
    call.set_return(emu, v);
}

fn ExAcquireFastMutex(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let m = api_try!(emu, call.arg(emu, 0));
    let thread = emu
        .kernel
        .as_ref()
        .map(|k| k.win.current_thread)
        .unwrap_or(0);
    emu.maps.write_dword(m + FAST_MUTEX_COUNT, 0);
    emu.maps.write_qword(m + FAST_MUTEX_OWNER, thread);
    call.set_return(emu, 0);
}

fn ExReleaseFastMutex(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let m = api_try!(emu, call.arg(emu, 0));
    emu.maps.write_dword(m + FAST_MUTEX_COUNT, 1);
    emu.maps.write_qword(m + FAST_MUTEX_OWNER, 0);
    call.set_return(emu, 0);
}

fn resource_rundown_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    if symbol == "ExInitializeRundownProtection" || symbol == "ExReInitializeRundownProtection" {
        let p = api_try!(emu, call.arg(emu, 0));
        emu.maps.write_qword(p, 0);
    }
    call.set_return(emu, 0);
}

fn ExAllocateCacheAwareRundownProtection(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let p = emu.kernel_alloc(
        crate::kernel::heap::Region::Slab,
        0x40,
        "rundown",
        symbol,
        true,
    );
    call.set_return(emu, p);
}

fn KeInitializeEvent(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let ev = api_try!(emu, call.arg(emu, 0));
    let t = api_try!(emu, call.arg_u8(emu, 1));
    let state = api_try!(emu, call.arg_u32(emu, 2));
    emu.maps.write_bytes(ev, &[0u8; KEVENT_SIZE as usize]);
    emu.maps.write_byte(ev + DISPATCHER_TYPE, t);
    emu.maps.write_dword(ev + DISPATCHER_SIGNAL_STATE, state);
    emu.maps.write_qword(ev + 8, ev + 8);
    emu.maps.write_qword(ev + 0x10, ev + 8);
    call.set_return(emu, 0);
}

fn KeSetEvent(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let ev = api_try!(emu, call.arg(emu, 0));
    let old = emu
        .maps
        .read_dword(ev + DISPATCHER_SIGNAL_STATE)
        .unwrap_or(0);
    emu.maps.write_dword(ev + DISPATCHER_SIGNAL_STATE, 1);
    call.set_return(emu, old as u64);
}

fn KeResetEvent(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let ev = api_try!(emu, call.arg(emu, 0));
    let old = emu
        .maps
        .read_dword(ev + DISPATCHER_SIGNAL_STATE)
        .unwrap_or(0);
    emu.maps.write_dword(ev + DISPATCHER_SIGNAL_STATE, 0);
    call.set_return(emu, old as u64);
}

fn KeReadStateEvent(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let obj = api_try!(emu, call.arg(emu, 0));
    let v = emu
        .maps
        .read_dword(obj + DISPATCHER_SIGNAL_STATE)
        .unwrap_or(0);
    call.set_return(emu, v as u64);
}

fn KeInitializeMutex(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let m = api_try!(emu, call.arg(emu, 0));
    emu.maps.write_bytes(m, &[0u8; 0x38]);
    emu.maps.write_byte(m + DISPATCHER_TYPE, MUTANT_OBJECT);
    emu.maps.write_dword(m + DISPATCHER_SIGNAL_STATE, 1);
    call.set_return(emu, 0);
}

fn KeReleaseMutex(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let m = api_try!(emu, call.arg(emu, 0));
    let old = emu
        .maps
        .read_dword(m + DISPATCHER_SIGNAL_STATE)
        .unwrap_or(0);
    emu.maps.write_dword(m + DISPATCHER_SIGNAL_STATE, 1);
    call.set_return(emu, old as u64);
}

fn KeInitializeSemaphore(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let s = api_try!(emu, call.arg(emu, 0));
    let count = api_try!(emu, call.arg_u32(emu, 1));
    let limit = api_try!(emu, call.arg_u32(emu, 2));
    emu.maps.write_bytes(s, &[0u8; 0x20]);
    emu.maps.write_byte(s + DISPATCHER_TYPE, SEMAPHORE_OBJECT);
    emu.maps.write_dword(s + DISPATCHER_SIGNAL_STATE, count);
    emu.maps.write_dword(s + 0x18, limit);
    call.set_return(emu, 0);
}

fn KeReleaseSemaphore(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let s = api_try!(emu, call.arg(emu, 0));
    let adj = api_try!(emu, call.arg_u32(emu, 2));
    let old = emu
        .maps
        .read_dword(s + DISPATCHER_SIGNAL_STATE)
        .unwrap_or(0);
    emu.maps
        .write_dword(s + DISPATCHER_SIGNAL_STATE, old.wrapping_add(adj));
    call.set_return(emu, old as u64);
}

fn KeWaitForSingleObject(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let obj = api_try!(emu, call.arg(emu, 0));
    let timeout = api_try!(emu, call.arg(emu, 4));
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
        call.set_return(emu, STATUS_SUCCESS);
    } else if timeout != 0 {
        // Nothing else runs, so the wait can only time out.
        call.set_return(emu, STATUS_TIMEOUT);
    } else {
        // An infinite wait on an unsignaled object would hang a real
        // kernel; here the pending work is what would signal it, so
        // treat the wait as satisfied and let the driver continue.
        log::info!("{}: object 0x{:x} not signaled, continuing", symbol, obj);
        call.set_return(emu, STATUS_SUCCESS);
    }
}

fn wait_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, STATUS_SUCCESS);
}

fn KePulseEvent(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let ev = api_try!(emu, call.arg(emu, 0));
    let old = emu
        .maps
        .read_dword(ev + DISPATCHER_SIGNAL_STATE)
        .unwrap_or(0);
    call.set_return(emu, old as u64);
}

fn KeInitializeDpc(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dpc = api_try!(emu, call.arg(emu, 0));
    let routine = api_try!(emu, call.arg(emu, 1));
    let ctx = api_try!(emu, call.arg(emu, 2));
    emu.maps.write_bytes(dpc, &[0u8; KDPC_SIZE as usize]);
    emu.maps.write_byte(dpc + KDPC_TYPE, KDPC_TYPE_VALUE);
    emu.maps.write_qword(dpc + KDPC_DEFERRED_ROUTINE, routine);
    emu.maps.write_qword(dpc + KDPC_DEFERRED_CONTEXT, ctx);
    call.set_return(emu, 0);
}

fn KeInsertQueueDpc(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dpc = api_try!(emu, call.arg(emu, 0));
    let a1 = api_try!(emu, call.arg(emu, 1));
    let a2 = api_try!(emu, call.arg(emu, 2));
    queue_dpc(emu, "dpc", dpc, a1, a2);
    call.set_return(emu, 1);
}

fn KeRemoveQueueDpc(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let dpc = api_try!(emu, call.arg(emu, 0));
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
    call.set_return(emu, removed as u64);
}

fn KeInitializeTimer(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let t = api_try!(emu, call.arg(emu, 0));
    let kind = if symbol == "KeInitializeTimerEx" {
        api_try!(emu, call.arg_u8(emu, 1))
    } else {
        0
    };
    emu.maps.write_bytes(t, &[0u8; 0x40]);
    emu.maps
        .write_byte(t + DISPATCHER_TYPE, NOTIFICATION_TIMER + kind);
    emu.maps.write_qword(t + 8, t + 8);
    emu.maps.write_qword(t + 0x10, t + 8);
    call.set_return(emu, 0);
}

fn KeSetTimer(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let t = api_try!(emu, call.arg(emu, 0));
    let dpc = if symbol == "KeSetTimer" {
        api_try!(emu, call.arg(emu, 2))
    } else {
        api_try!(emu, call.arg(emu, 3))
    };
    let period = if symbol == "KeSetTimer" {
        0
    } else {
        api_try!(emu, call.arg_u32(emu, 2))
    };
    let was_set = emu.maps.read_qword(t + KTIMER_DPC).unwrap_or(0) != 0;
    emu.maps.write_qword(t + KTIMER_DPC, dpc);
    emu.maps.write_dword(t + KTIMER_PERIOD, period);
    // The timer "fires": its DPC is queued once. A periodic timer is
    // not re-armed, which keeps a driver with a heartbeat from
    // queueing forever.
    emu.maps.write_dword(t + DISPATCHER_SIGNAL_STATE, 1);
    queue_dpc(emu, "timer", dpc, 0, 0);
    call.set_return(emu, was_set as u64);
}

fn KeCancelTimer(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let t = api_try!(emu, call.arg(emu, 0));
    let dpc = emu.maps.read_qword(t + KTIMER_DPC).unwrap_or(0);
    let (routine, _) = dpc_routine(emu, dpc);
    let was_set = dpc != 0;
    emu.maps.write_qword(t + KTIMER_DPC, 0);
    if let Some(k) = emu.kernel.as_mut() {
        k.deferred
            .retain(|d| d.kind != "timer" || d.func != routine);
    }
    call.set_return(emu, was_set as u64);
}

fn KeInsertQueueApc(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    // KeInsertQueueApc(Apc, SystemArgument1, SystemArgument2, Increment):
    // the kernel routine was stored by KeInitializeApc at +0x30
    // (KernelRoutine) with NormalRoutine at +0x40, NormalContext +0x48.
    let apc = api_try!(emu, call.arg(emu, 0));
    let kernel_routine = emu.maps.read_qword(apc + 0x30).unwrap_or(0);
    let normal = emu.maps.read_qword(apc + 0x40).unwrap_or(0);
    let ctx = emu.maps.read_qword(apc + 0x48).unwrap_or(0);
    let a1 = api_try!(emu, call.arg(emu, 1));
    let a2 = api_try!(emu, call.arg(emu, 2));
    if kernel_routine != 0 {
        emu.kernel_defer(DeferredCall {
            kind: "apc".to_string(),
            func: kernel_routine,
            args: vec![apc, normal, ctx, a1, a2],
        });
    }
    call.set_return(emu, 1);
}

fn KeRegisterNmiCallback(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg(emu, 0));
    let ctx = api_try!(emu, call.arg(emu, 1));
    if let Some(k) = emu.kernel.as_mut() {
        k.win.callbacks.push(RegisteredCallback {
            api: symbol.to_string(),
            func,
            context: ctx,
        });
    }
    let h = new_handle(emu, symbol.to_string());
    call.set_return(
        emu,
        if symbol == "KeRegisterBugCheckCallback" {
            1
        } else {
            h
        },
    );
}

fn KeIpiGenericCall(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg(emu, 0));
    let ctx = api_try!(emu, call.arg(emu, 1));
    emu.kernel_privop("ipi_call", func, ctx, 0);
    emu.kernel_tail_call(func, &[ctx]);
}

fn KeExpandKernelStackAndCallout(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let func = api_try!(emu, call.arg(emu, 0));
    let param = api_try!(emu, call.arg(emu, 1));
    emu.kernel_tail_call(func, &[param]);
}

fn KeGenericCallDpc(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let routine = api_try!(emu, call.arg(emu, 0));
    let ctx = api_try!(emu, call.arg(emu, 1));
    emu.kernel_defer(DeferredCall {
        kind: "generic_dpc".to_string(),
        func: routine,
        args: vec![0, ctx, 0, 0],
    });
    call.set_return(emu, 0);
}

fn processor_number_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    if symbol == "KeGetCurrentProcessorNumberEx" {
        let out = api_try!(emu, call.arg(emu, 0));
        if out != 0 {
            emu.maps.write_dword(out, 0);
        }
    }
    if symbol == "KeGetProcessorNumberFromIndex" {
        let out = api_try!(emu, call.arg(emu, 1));
        if out != 0 {
            emu.maps.write_dword(out, 0);
        }
    }
    call.set_return(emu, 0);
}

fn processor_affinity_stub(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    if symbol == "KeQueryActiveProcessorCount" {
        let out = api_try!(emu, call.arg(emu, 0));
        if out != 0 {
            emu.maps.write_qword(out, 1);
        }
    }
    call.set_return(emu, 1);
}

fn KeQueryPerformanceCounter(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let freq = api_try!(emu, call.arg(emu, 0));
    if freq != 0 {
        emu.maps.write_qword(freq, 10_000_000);
    }
    call.set_return(emu, emu.pos.wrapping_mul(10));
}

fn KeQuerySystemTime(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let t = system_time_now();
    write_out_qword(emu, out, t);
    call.set_return(emu, 0);
}

fn KeQueryInterruptTime(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let t = emu.pos.wrapping_mul(100);
    let out = api_try!(emu, call.arg(emu, 0));
    if symbol.ends_with("Precise") {
        write_out_qword(emu, out, t);
    }
    call.set_return(emu, t);
}

fn KeQueryTickCount(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    write_out_qword(emu, out, emu.pos / 1000);
    call.set_return(emu, 0);
}

fn KeQueryTimeIncrement(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 156_250);
}

fn KeGetCurrentThread(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    let t = emu
        .kernel
        .as_ref()
        .map(|k| k.win.current_thread)
        .unwrap_or(0);
    call.set_return(emu, t);
}

fn KeGetRecommendedSharedDataAlignment(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 64);
}

fn thread_query_stub(emu: &mut Emu, _symbol: &str) {
    let call = WinApi(Stdcall);
    call.set_return(emu, 8);
}

fn KeBugCheck(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let code = api_try!(emu, call.arg(emu, 0));
    let (a, b, c, d) = if symbol == "KeBugCheckEx" {
        (
            api_try!(emu, call.arg(emu, 1)),
            api_try!(emu, call.arg(emu, 2)),
            api_try!(emu, call.arg(emu, 3)),
            api_try!(emu, call.arg(emu, 4)),
        )
    } else {
        (0, 0, 0, 0)
    };
    let line = format!(
        "*** STOP: 0x{:08X} (0x{:x}, 0x{:x}, 0x{:x}, 0x{:x})",
        code, a, b, c, d
    );
    emu.kernel_abort(format!("{} {}", symbol, line));
}

fn ExRaiseStatus(emu: &mut Emu, symbol: &str) {
    let call = WinApi(Stdcall);
    let status = if symbol == "ExRaiseStatus" {
        api_try!(emu, call.arg(emu, 0))
    } else {
        STATUS_ACCESS_VIOLATION
    };
    emu.kernel_abort(format!(
        "{}(0x{:x}) — no SEH dispatch in kernel mode",
        symbol, status
    ));
}
