use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod create_mutex;
pub use create_mutex::{CreateMutexA, CreateMutexW};

mod create_event;
pub use create_event::{CreateEventA, CreateEventW};

mod enter_critical_section;
pub use enter_critical_section::EnterCriticalSection;

mod fls_alloc;
pub use fls_alloc::FlsAlloc;

mod fls_get_value;
pub use fls_get_value::FlsGetValue;

mod fls_get_value2;
pub use fls_get_value2::FlsGetValue2;

mod fls_set_value;
pub use fls_set_value::FlsSetValue;

mod init_once_begin_initialize;
pub use init_once_begin_initialize::InitOnceBeginInitialize;

mod initialize_critical_section;
pub use initialize_critical_section::InitializeCriticalSection;

mod initialize_critical_section_and_spin_count;
pub use initialize_critical_section_and_spin_count::InitializeCriticalSectionAndSpinCount;

mod initialize_critical_section_ex;
pub use initialize_critical_section_ex::InitializeCriticalSectionEx;

mod leave_critical_section;
pub use leave_critical_section::LeaveCriticalSection;

mod reset_event;
pub use reset_event::ResetEvent;

mod tls_alloc;
pub use tls_alloc::TlsAlloc;

mod tls_free;
pub use tls_free::TlsFree;

mod tls_get_value;
pub use tls_get_value::TlsGetValue;

mod tls_get_value2;
pub use tls_get_value2::TlsGetValue2;

mod tls_set_value;
pub use tls_set_value::TlsSetValue;

mod wait_for_single_object;
pub use wait_for_single_object::WaitForSingleObject;

mod wait_for_multiple_objects;
pub use wait_for_multiple_objects::WaitForMultipleObjects;

mod initialize_srw_lock;
pub use initialize_srw_lock::InitializeSRWLock;

mod acquire_srw_lock_exclusive;
pub use acquire_srw_lock_exclusive::AcquireSRWLockExclusive;

mod acquire_srw_lock_shared;
pub use acquire_srw_lock_shared::AcquireSRWLockShared;

mod release_srw_lock_exclusive;
pub use release_srw_lock_exclusive::ReleaseSRWLockExclusive;

mod release_srw_lock_shared;
pub use release_srw_lock_shared::ReleaseSRWLockShared;

mod try_acquire_srw_lock_exclusive;
pub use try_acquire_srw_lock_exclusive::TryAcquireSRWLockExclusive;

mod try_acquire_srw_lock_shared;
pub use try_acquire_srw_lock_shared::TryAcquireSRWLockShared;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "AcquireSRWLockExclusive" => kernel32::AcquireSRWLockExclusive(emu),
        "AcquireSRWLockShared" => kernel32::AcquireSRWLockShared(emu),
        "CreateEventA" => kernel32::CreateEventA(emu),
        "CreateEventW" => kernel32::CreateEventW(emu),
        "CreateMutexA" => kernel32::CreateMutexA(emu),
        "CreateMutexW" => kernel32::CreateMutexW(emu),
        "EnterCriticalSection" => kernel32::EnterCriticalSection(emu),
        "InitializeCriticalSection" => kernel32::InitializeCriticalSection(emu),
        "InitializeCriticalSectionAndSpinCount" => {
            kernel32::InitializeCriticalSectionAndSpinCount(emu)
        }
        "InitializeCriticalSectionEx" => kernel32::InitializeCriticalSectionEx(emu),
        "InitializeSRWLock" => kernel32::InitializeSRWLock(emu),
        "InitOnceBeginInitialize" => kernel32::InitOnceBeginInitialize(emu),
        "LeaveCriticalSection" => kernel32::LeaveCriticalSection(emu),
        "ResetEvent" => kernel32::ResetEvent(emu),
        "SetEvent" => kernel32::SetEvent(emu),
        "ReleaseSRWLockExclusive" => kernel32::ReleaseSRWLockExclusive(emu),
        "ReleaseSRWLockShared" => kernel32::ReleaseSRWLockShared(emu),
        "Sleep" => kernel32::Sleep(emu),
        "TryAcquireSRWLockExclusive" => kernel32::TryAcquireSRWLockExclusive(emu),
        "TryAcquireSRWLockShared" => kernel32::TryAcquireSRWLockShared(emu),
        "WaitForMultipleObjects" => kernel32::WaitForMultipleObjects(emu),
        "WaitForSingleObject" => kernel32::WaitForSingleObject(emu),
        "CreateSemaphoreA" => kernel32::CreateSemaphoreA(emu),
        "CreateSemaphoreW" => kernel32::CreateSemaphoreW(emu),
        "CreateSemaphoreExA" => kernel32::CreateSemaphoreExA(emu),
        "CreateSemaphoreExW" => kernel32::CreateSemaphoreExW(emu),
        "CreateWaitableTimerA" => kernel32::CreateWaitableTimerA(emu),
        "CreateWaitableTimerW" => kernel32::CreateWaitableTimerW(emu),
        "CreateWaitableTimerExA" => kernel32::CreateWaitableTimerExA(emu),
        "CreateWaitableTimerExW" => kernel32::CreateWaitableTimerExW(emu),
        "OpenMutexA" => kernel32::OpenMutexA(emu),
        "OpenMutexW" => kernel32::OpenMutexW(emu),
        "InterlockedIncrement" => kernel32::InterlockedIncrement(emu, addr),
        "InterlockedDecrement" => kernel32::InterlockedDecrement(emu, addr),
        "InterlockedExchange" => kernel32::InterlockedExchange(emu, addr),
        "InterlockedCompareExchange" => kernel32::InterlockedCompareExchange(emu, addr),
        "DeleteCriticalSection" => kernel32::DeleteCriticalSection(emu),
        "InitializeConditionVariable" => kernel32::InitializeConditionVariable(emu),
        "WakeConditionVariable" | "WakeAllConditionVariable" => {
            kernel32::WakeConditionVariable(emu, api)
        }
        "SleepConditionVariableSRW" => kernel32::SleepConditionVariableSRW(emu),
        "SleepConditionVariableCS" => kernel32::SleepConditionVariableCS(emu),
        "SleepEx" => kernel32::SleepEx(emu),
        "WaitForSingleObjectEx" => kernel32::WaitForSingleObjectEx(emu),
        "WaitForMultipleObjectsEx" => kernel32::WaitForMultipleObjectsEx(emu),
        _ => return false,
    }
    true
}
