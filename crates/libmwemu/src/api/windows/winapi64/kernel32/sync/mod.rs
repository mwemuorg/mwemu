use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

#[path = "../create_mutex.rs"]
mod create_mutex;
pub use create_mutex::*;

#[path = "../create_event.rs"]
mod create_event;
pub use create_event::*;

#[path = "../enter_critical_section.rs"]
mod enter_critical_section;
pub use enter_critical_section::*;

#[path = "../fls_alloc.rs"]
mod fls_alloc;
pub use fls_alloc::*;

#[path = "../fls_get_value.rs"]
mod fls_get_value;
pub use fls_get_value::*;

#[path = "../fls_get_value2.rs"]
mod fls_get_value2;
pub use fls_get_value2::*;

#[path = "../fls_set_value.rs"]
mod fls_set_value;
pub use fls_set_value::*;

#[path = "../init_once_begin_initialize.rs"]
mod init_once_begin_initialize;
pub use init_once_begin_initialize::*;

#[path = "../initialize_critical_section.rs"]
mod initialize_critical_section;
pub use initialize_critical_section::*;

#[path = "../initialize_critical_section_and_spin_count.rs"]
mod initialize_critical_section_and_spin_count;
pub use initialize_critical_section_and_spin_count::*;

#[path = "../initialize_critical_section_ex.rs"]
mod initialize_critical_section_ex;
pub use initialize_critical_section_ex::*;

#[path = "../leave_critical_section.rs"]
mod leave_critical_section;
pub use leave_critical_section::*;

#[path = "../reset_event.rs"]
mod reset_event;
pub use reset_event::*;

#[path = "../tls_alloc.rs"]
mod tls_alloc;
pub use tls_alloc::*;

#[path = "../tls_free.rs"]
mod tls_free;
pub use tls_free::*;

#[path = "../tls_get_value.rs"]
mod tls_get_value;
pub use tls_get_value::*;

#[path = "../tls_get_value2.rs"]
mod tls_get_value2;
pub use tls_get_value2::*;

#[path = "../tls_set_value.rs"]
mod tls_set_value;
pub use tls_set_value::*;

#[path = "../wait_for_single_object.rs"]
mod wait_for_single_object;
pub use wait_for_single_object::*;

#[path = "../wait_for_multiple_objects.rs"]
mod wait_for_multiple_objects;
pub use wait_for_multiple_objects::*;

#[path = "../initialize_srw_lock.rs"]
mod initialize_srw_lock;
pub use initialize_srw_lock::*;

#[path = "../acquire_srw_lock_exclusive.rs"]
mod acquire_srw_lock_exclusive;
pub use acquire_srw_lock_exclusive::*;

#[path = "../acquire_srw_lock_shared.rs"]
mod acquire_srw_lock_shared;
pub use acquire_srw_lock_shared::*;

#[path = "../release_srw_lock_exclusive.rs"]
mod release_srw_lock_exclusive;
pub use release_srw_lock_exclusive::*;

#[path = "../release_srw_lock_shared.rs"]
mod release_srw_lock_shared;
pub use release_srw_lock_shared::*;

#[path = "../try_acquire_srw_lock_exclusive.rs"]
mod try_acquire_srw_lock_exclusive;
pub use try_acquire_srw_lock_exclusive::*;

#[path = "../try_acquire_srw_lock_shared.rs"]
mod try_acquire_srw_lock_shared;
pub use try_acquire_srw_lock_shared::*;

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
        "CreateSemaphoreA" | "CreateSemaphoreW" => kernel32::CreateSemaphoreA(emu, api),
        "CreateSemaphoreExA" => kernel32::CreateSemaphoreExA(emu),
        "CreateSemaphoreExW" => kernel32::CreateSemaphoreExW(emu),
        "CreateWaitableTimerA" | "CreateWaitableTimerW" => kernel32::CreateWaitableTimerA(emu, api),
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
