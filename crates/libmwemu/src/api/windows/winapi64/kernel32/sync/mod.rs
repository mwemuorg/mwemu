pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    AcquireSRWLockExclusive, AcquireSRWLockShared, EnterCriticalSection, FlsAlloc, FlsGetValue,
    FlsGetValue2, FlsSetValue, InitOnceBeginInitialize, InitializeCriticalSection,
    InitializeCriticalSectionAndSpinCount, InitializeCriticalSectionEx, InitializeSRWLock,
    LeaveCriticalSection, ReleaseSRWLockExclusive, ReleaseSRWLockShared, TlsGetValue2,
    TryAcquireSRWLockExclusive, TryAcquireSRWLockShared,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

pub use crate::winapi::common::threads::{TlsAlloc, TlsFree, TlsGetValue, TlsSetValue};

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "AcquireSRWLockExclusive" => kernel32::AcquireSRWLockExclusive(emu),
        "AcquireSRWLockShared" => kernel32::AcquireSRWLockShared(emu),
        "EnterCriticalSection" => kernel32::EnterCriticalSection(emu),
        "InitializeCriticalSection" => kernel32::InitializeCriticalSection(emu),
        "InitializeCriticalSectionAndSpinCount" => kernel32::InitializeCriticalSectionAndSpinCount(emu),
        "InitializeCriticalSectionEx" => kernel32::InitializeCriticalSectionEx(emu),
        "InitializeSRWLock" => kernel32::InitializeSRWLock(emu),
        "InitOnceBeginInitialize" => kernel32::InitOnceBeginInitialize(emu),
        "LeaveCriticalSection" => kernel32::LeaveCriticalSection(emu),
        "ReleaseSRWLockExclusive" => kernel32::ReleaseSRWLockExclusive(emu),
        "ReleaseSRWLockShared" => kernel32::ReleaseSRWLockShared(emu),
        "Sleep" => kernel32::Sleep(emu),
        "TryAcquireSRWLockExclusive" => kernel32::TryAcquireSRWLockExclusive(emu),
        "TryAcquireSRWLockShared" => kernel32::TryAcquireSRWLockShared(emu),
        "InterlockedIncrement" => kernel32::InterlockedIncrement(emu, addr),
        "InterlockedDecrement" => kernel32::InterlockedDecrement(emu, addr),
        "InterlockedExchange" => kernel32::InterlockedExchange(emu, addr),
        "InterlockedCompareExchange" => kernel32::InterlockedCompareExchange(emu, addr),
        "DeleteCriticalSection" => kernel32::DeleteCriticalSection(emu),
        "InitializeConditionVariable" => kernel32::InitializeConditionVariable(emu),
        "WakeConditionVariable" | "WakeAllConditionVariable" => kernel32::WakeConditionVariable(emu, api),
        "SleepConditionVariableSRW" => kernel32::SleepConditionVariableSRW(emu),
        "SleepConditionVariableCS" => kernel32::SleepConditionVariableCS(emu),
        "SleepEx" => kernel32::SleepEx(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
