pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    AcquireSRWLockExclusive, AcquireSRWLockShared, EnterCriticalSection, FlsAlloc, FlsGetValue,
    FlsSetValue, InitializeCriticalSection, InitializeCriticalSectionAndSpinCount,
    InitializeCriticalSectionEx, InitializeSRWLock, InterlockedCompareExchange,
    InterlockedDecrement, InterlockedExchange, InterlockedExchangeAdd, InterlockedIncrement,
    LeaveCriticalSection, ReleaseSRWLockExclusive, ReleaseSRWLockShared,
    TryAcquireSRWLockExclusive, TryAcquireSRWLockShared,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::fallback::unsupported_exports;
use crate::winapi::winapi32::kernel32;

pub use crate::winapi::common::threads::{TlsAlloc, TlsFree, TlsGetValue, TlsSetValue};

unsupported_exports!(
    "kernel32",
    RegisterWaitForInputIdle,
    RegisterWaitUntilOOBECompleted,
    SetMessageWaitingIndicator,
    UnregisterWait,
    UnregisterWaitUntilOOBECompleted
);

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "InitializeSRWLock" => kernel32::InitializeSRWLock(emu),
        "AcquireSRWLockExclusive" => kernel32::AcquireSRWLockExclusive(emu),
        "AcquireSRWLockShared" => kernel32::AcquireSRWLockShared(emu),
        "ReleaseSRWLockExclusive" => kernel32::ReleaseSRWLockExclusive(emu),
        "ReleaseSRWLockShared" => kernel32::ReleaseSRWLockShared(emu),
        "TryAcquireSRWLockExclusive" => kernel32::TryAcquireSRWLockExclusive(emu),
        "TryAcquireSRWLockShared" => kernel32::TryAcquireSRWLockShared(emu),
        "EnterCriticalSection" => kernel32::EnterCriticalSection(emu),
        "InitializeCriticalSection" => kernel32::InitializeCriticalSection(emu),
        "InitializeCriticalSectionAndSpinCount" => kernel32::InitializeCriticalSectionAndSpinCount(emu),
        "InitializeCriticalSectionEx" => kernel32::InitializeCriticalSectionEx(emu),
        "InterlockedIncrement" => kernel32::InterlockedIncrement(emu),
        "LeaveCriticalSection" => kernel32::LeaveCriticalSection(emu),
        "Sleep" => kernel32::Sleep(emu),
        "BaseWriteErrorElevationRequiredEvent" => kernel32::BaseWriteErrorElevationRequiredEvent(emu),
        "CptpQuirksInitOnce" => kernel32::CptpQuirksInitOnce(emu),
        "InitOnceGetStringTableOffset" => kernel32::InitOnceGetStringTableOffset(emu),
        "Initialize16BitCriticalSection" => kernel32::Initialize16BitCriticalSection(emu),
        "InterlockedCompareExchange" => kernel32::InterlockedCompareExchange(emu),
        "InterlockedDecrement" => kernel32::InterlockedDecrement(emu),
        "InterlockedExchange" => kernel32::InterlockedExchange(emu),
        "InterlockedExchangeAdd" => kernel32::InterlockedExchangeAdd(emu),
        "UnregisterWait" => kernel32::UnregisterWait(emu),
        "UnregisterWaitUntilOOBECompleted" => kernel32::UnregisterWaitUntilOOBECompleted(emu),
        "DeleteCriticalSection" => kernel32::DeleteCriticalSection(emu),
        "InitOnceBeginInitialize" => kernel32::InitOnceBeginInitialize(emu),
        "InitOnceComplete" => kernel32::InitOnceComplete(emu),
        "InitOnceExecuteOnce" => kernel32::InitOnceExecuteOnce(emu),
        "InitOnceInitialize" => kernel32::InitOnceInitialize(emu),
        "InitializeConditionVariable" => kernel32::InitializeConditionVariable(emu),
        "InitializeSListHead" => kernel32::InitializeSListHead(emu),
        "InterlockedCompareExchange64" => kernel32::InterlockedCompareExchange64(emu),
        "InterlockedFlushSList" => kernel32::InterlockedFlushSList(emu),
        "InterlockedPopEntrySList" => kernel32::InterlockedPopEntrySList(emu),
        "InterlockedPushEntrySList" => kernel32::InterlockedPushEntrySList(emu),
        "InterlockedPushListSList" => kernel32::InterlockedPushListSList(emu),
        "InterlockedPushListSListEx" => kernel32::InterlockedPushListSListEx(emu),
        "LeaveCriticalSectionWhenCallbackReturns" => kernel32::LeaveCriticalSectionWhenCallbackReturns(emu),
        "QueryDepthSList" => kernel32::QueryDepthSList(emu),
        "ReleaseMutexWhenCallbackReturns" => kernel32::ReleaseMutexWhenCallbackReturns(emu),
        "ReleaseSemaphoreWhenCallbackReturns" => kernel32::ReleaseSemaphoreWhenCallbackReturns(emu),
        "SetCriticalSectionSpinCount" => kernel32::SetCriticalSectionSpinCount(emu),
        "SetEventWhenCallbackReturns" => kernel32::SetEventWhenCallbackReturns(emu),
        "SleepConditionVariableCS" => kernel32::SleepConditionVariableCS(emu),
        "SleepConditionVariableSRW" => kernel32::SleepConditionVariableSRW(emu),
        "SleepEx" => kernel32::SleepEx(emu),
        "TryEnterCriticalSection" => kernel32::TryEnterCriticalSection(emu),
        "WaitForThreadpoolIoCallbacks" => kernel32::WaitForThreadpoolIoCallbacks(emu),
        "WaitForThreadpoolTimerCallbacks" => kernel32::WaitForThreadpoolTimerCallbacks(emu),
        "WaitForThreadpoolWaitCallbacks" => kernel32::WaitForThreadpoolWaitCallbacks(emu),
        "WaitForThreadpoolWorkCallbacks" => kernel32::WaitForThreadpoolWorkCallbacks(emu),
        "WakeAllConditionVariable" => kernel32::WakeAllConditionVariable(emu),
        "WakeConditionVariable" => kernel32::WakeConditionVariable(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

unsupported_exports!(
    "kernel32",
    CptpQuirksInitOnce,
    InitOnceGetStringTableOffset,
    Initialize16BitCriticalSection,
    OOBECompleteWnfWaitCallback,
    TraceLoggingRegisterEx_EventRegister_EventSetInformation,
);
