use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

#[path = "../wait_named_pipe.rs"]
mod wait_named_pipe;
pub use wait_named_pipe::*;

#[path = "../open_mutex.rs"]
mod open_mutex;
pub use open_mutex::*;

#[path = "../create_waitable_timer_ex.rs"]
mod create_waitable_timer_ex;
pub use create_waitable_timer_ex::*;

#[path = "../create_semaphore_ex.rs"]
mod create_semaphore_ex;
pub use create_semaphore_ex::*;

#[path = "../create_semaphore.rs"]
mod create_semaphore;
pub use create_semaphore::*;

#[path = "../create_event.rs"]
mod create_event;
pub use create_event::*;

#[path = "../create_waitable_timer.rs"]
mod create_waitable_timer;
pub use create_waitable_timer::*;

#[path = "../create_mutex.rs"]
mod create_mutex;
pub use create_mutex::*;

#[path = "../enter_critical_section.rs"]
mod enter_critical_section;
pub use enter_critical_section::*;

#[path = "../fls_alloc.rs"]
mod fls_alloc;
pub use fls_alloc::*;

#[path = "../fls_get_value.rs"]
mod fls_get_value;
pub use fls_get_value::*;

#[path = "../fls_set_value.rs"]
mod fls_set_value;
pub use fls_set_value::*;

#[path = "../initialize_critical_section.rs"]
mod initialize_critical_section;
pub use initialize_critical_section::*;

#[path = "../initialize_critical_section_and_spin_count.rs"]
mod initialize_critical_section_and_spin_count;
pub use initialize_critical_section_and_spin_count::*;

#[path = "../initialize_critical_section_ex.rs"]
mod initialize_critical_section_ex;
pub use initialize_critical_section_ex::*;

#[path = "../interlocked_increment.rs"]
mod interlocked_increment;
pub use interlocked_increment::*;

#[path = "../leave_critical_section.rs"]
mod leave_critical_section;
pub use leave_critical_section::*;

#[path = "../tls_alloc.rs"]
mod tls_alloc;
pub use tls_alloc::*;

#[path = "../tls_free.rs"]
mod tls_free;
pub use tls_free::*;

#[path = "../tls_get_value.rs"]
mod tls_get_value;
pub use tls_get_value::*;

#[path = "../tls_set_value.rs"]
mod tls_set_value;
pub use tls_set_value::*;

#[path = "../wait_for_single_object.rs"]
mod wait_for_single_object;
pub use wait_for_single_object::*;

#[path = "../cptp_quirks_init_once.rs"]
mod cptp_quirks_init_once;
pub use cptp_quirks_init_once::*;

#[path = "../init_once_get_string_table_offset.rs"]
mod init_once_get_string_table_offset;
pub use init_once_get_string_table_offset::*;

#[path = "../initialize16_bit_critical_section.rs"]
mod initialize16_bit_critical_section;
pub use initialize16_bit_critical_section::*;

#[path = "../interlocked_compare_exchange.rs"]
mod interlocked_compare_exchange;
pub use interlocked_compare_exchange::*;

#[path = "../interlocked_decrement.rs"]
mod interlocked_decrement;
pub use interlocked_decrement::*;

#[path = "../interlocked_exchange.rs"]
mod interlocked_exchange;
pub use interlocked_exchange::*;

#[path = "../interlocked_exchange_add.rs"]
mod interlocked_exchange_add;
pub use interlocked_exchange_add::*;

#[path = "../oobe_complete_wnf_wait_callback.rs"]
mod oobe_complete_wnf_wait_callback;
pub use oobe_complete_wnf_wait_callback::*;

#[path = "../register_wait_for_input_idle.rs"]
mod register_wait_for_input_idle;
pub use register_wait_for_input_idle::*;

#[path = "../register_wait_until_oobe_completed.rs"]
mod register_wait_until_oobe_completed;
pub use register_wait_until_oobe_completed::*;

#[path = "../set_message_waiting_indicator.rs"]
mod set_message_waiting_indicator;
pub use set_message_waiting_indicator::*;

#[path = "../trace_logging_register_ex__event_register__event_set_information.rs"]
mod trace_logging_register_ex__event_register__event_set_information;
pub use trace_logging_register_ex__event_register__event_set_information::*;

#[path = "../unregister_wait.rs"]
mod unregister_wait;
pub use unregister_wait::*;

#[path = "../unregister_wait_until_oobe_completed.rs"]
mod unregister_wait_until_oobe_completed;
pub use unregister_wait_until_oobe_completed::*;

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

#[path = "../reset_event.rs"]
mod reset_event;
pub use reset_event::*;

#[path = "../wait_for_multiple_objects.rs"]
mod wait_for_multiple_objects;
pub use wait_for_multiple_objects::*;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "InitializeSRWLock" => kernel32::InitializeSRWLock(emu),
        "AcquireSRWLockExclusive" => kernel32::AcquireSRWLockExclusive(emu),
        "AcquireSRWLockShared" => kernel32::AcquireSRWLockShared(emu),
        "ReleaseSRWLockExclusive" => kernel32::ReleaseSRWLockExclusive(emu),
        "ReleaseSRWLockShared" => kernel32::ReleaseSRWLockShared(emu),
        "TryAcquireSRWLockExclusive" => kernel32::TryAcquireSRWLockExclusive(emu),
        "TryAcquireSRWLockShared" => kernel32::TryAcquireSRWLockShared(emu),
        "ResetEvent" => kernel32::ResetEvent(emu),
        "WaitForMultipleObjects" => kernel32::WaitForMultipleObjects(emu),
        "CreateEventA" => kernel32::CreateEventA(emu),
        "CreateMutexA" => kernel32::CreateMutexA(emu),
        "CreateMutexW" => kernel32::CreateMutexW(emu),
        "EnterCriticalSection" => kernel32::EnterCriticalSection(emu),
        "InitializeCriticalSection" => kernel32::InitializeCriticalSection(emu),
        "InitializeCriticalSectionAndSpinCount" => {
            kernel32::InitializeCriticalSectionAndSpinCount(emu)
        }
        "InitializeCriticalSectionEx" => kernel32::InitializeCriticalSectionEx(emu),
        "InterlockedIncrement" => kernel32::InterlockedIncrement(emu),
        "LeaveCriticalSection" => kernel32::LeaveCriticalSection(emu),
        "Sleep" => kernel32::Sleep(emu),
        "WaitForSingleObject" => kernel32::WaitForSingleObject(emu),
        "BaseWriteErrorElevationRequiredEvent" => {
            kernel32::BaseWriteErrorElevationRequiredEvent(emu)
        }
        "CptpQuirksInitOnce" => kernel32::CptpQuirksInitOnce(emu),
        "CreateSemaphoreA" => kernel32::CreateSemaphoreA(emu),
        "CreateSemaphoreExA" => kernel32::CreateSemaphoreExA(emu),
        "CreateWaitableTimerA" => kernel32::CreateWaitableTimerA(emu),
        "CreateWaitableTimerExA" => kernel32::CreateWaitableTimerExA(emu),
        "CreateWaitableTimerW" => kernel32::CreateWaitableTimerW(emu),
        "InitOnceGetStringTableOffset" => kernel32::InitOnceGetStringTableOffset(emu),
        "Initialize16BitCriticalSection" => kernel32::Initialize16BitCriticalSection(emu),
        "InterlockedCompareExchange" => kernel32::InterlockedCompareExchange(emu),
        "InterlockedDecrement" => kernel32::InterlockedDecrement(emu),
        "InterlockedExchange" => kernel32::InterlockedExchange(emu),
        "InterlockedExchangeAdd" => kernel32::InterlockedExchangeAdd(emu),
        "OpenMutexA" => kernel32::OpenMutexA(emu),
        "UnregisterWait" => kernel32::UnregisterWait(emu),
        "UnregisterWaitUntilOOBECompleted" => kernel32::UnregisterWaitUntilOOBECompleted(emu),
        "CancelWaitableTimer" => kernel32::CancelWaitableTimer(emu),
        "CreateEventExA" => kernel32::CreateEventExA(emu),
        "CreateEventExW" => kernel32::CreateEventExW(emu),
        "CreateEventW" => kernel32::CreateEventW(emu),
        "CreateMutexExA" => kernel32::CreateMutexExA(emu),
        "CreateMutexExW" => kernel32::CreateMutexExW(emu),
        "CreateSemaphoreExW" => kernel32::CreateSemaphoreExW(emu),
        "CreateSemaphoreW" => kernel32::CreateSemaphoreW(emu),
        "CreateWaitableTimerExW" => kernel32::CreateWaitableTimerExW(emu),
        "DeleteCriticalSection" => kernel32::DeleteCriticalSection(emu),
        "DeleteSynchronizationBarrier" => kernel32::DeleteSynchronizationBarrier(emu),
        "EnterSynchronizationBarrier" => kernel32::EnterSynchronizationBarrier(emu),
        "InitOnceBeginInitialize" => kernel32::InitOnceBeginInitialize(emu),
        "InitOnceComplete" => kernel32::InitOnceComplete(emu),
        "InitOnceExecuteOnce" => kernel32::InitOnceExecuteOnce(emu),
        "InitOnceInitialize" => kernel32::InitOnceInitialize(emu),
        "InitializeConditionVariable" => kernel32::InitializeConditionVariable(emu),
        "InitializeSListHead" => kernel32::InitializeSListHead(emu),
        "InitializeSynchronizationBarrier" => kernel32::InitializeSynchronizationBarrier(emu),
        "InterlockedCompareExchange64" => kernel32::InterlockedCompareExchange64(emu),
        "InterlockedFlushSList" => kernel32::InterlockedFlushSList(emu),
        "InterlockedPopEntrySList" => kernel32::InterlockedPopEntrySList(emu),
        "InterlockedPushEntrySList" => kernel32::InterlockedPushEntrySList(emu),
        "InterlockedPushListSList" => kernel32::InterlockedPushListSList(emu),
        "InterlockedPushListSListEx" => kernel32::InterlockedPushListSListEx(emu),
        "LeaveCriticalSectionWhenCallbackReturns" => {
            kernel32::LeaveCriticalSectionWhenCallbackReturns(emu)
        }
        "OpenEventA" => kernel32::OpenEventA(emu),
        "OpenEventW" => kernel32::OpenEventW(emu),
        "OpenMutexW" => kernel32::OpenMutexW(emu),
        "OpenSemaphoreA" => kernel32::OpenSemaphoreA(emu),
        "OpenSemaphoreW" => kernel32::OpenSemaphoreW(emu),
        "OpenWaitableTimerA" => kernel32::OpenWaitableTimerA(emu),
        "OpenWaitableTimerW" => kernel32::OpenWaitableTimerW(emu),
        "PulseEvent" => kernel32::PulseEvent(emu),
        "QueryDepthSList" => kernel32::QueryDepthSList(emu),
        "ReleaseMutex" => kernel32::ReleaseMutex(emu),
        "ReleaseMutexWhenCallbackReturns" => kernel32::ReleaseMutexWhenCallbackReturns(emu),
        "ReleaseSemaphore" => kernel32::ReleaseSemaphore(emu),
        "ReleaseSemaphoreWhenCallbackReturns" => kernel32::ReleaseSemaphoreWhenCallbackReturns(emu),
        "SetCriticalSectionSpinCount" => kernel32::SetCriticalSectionSpinCount(emu),
        "SetEvent" => kernel32::SetEvent(emu),
        "SetEventWhenCallbackReturns" => kernel32::SetEventWhenCallbackReturns(emu),
        "SetWaitableTimer" => kernel32::SetWaitableTimer(emu),
        "SetWaitableTimerEx" => kernel32::SetWaitableTimerEx(emu),
        "SignalObjectAndWait" => kernel32::SignalObjectAndWait(emu),
        "SleepConditionVariableCS" => kernel32::SleepConditionVariableCS(emu),
        "SleepConditionVariableSRW" => kernel32::SleepConditionVariableSRW(emu),
        "SleepEx" => kernel32::SleepEx(emu),
        "TryEnterCriticalSection" => kernel32::TryEnterCriticalSection(emu),
        "UnregisterWaitEx" => kernel32::UnregisterWaitEx(emu),
        "WaitForMultipleObjectsEx" => kernel32::WaitForMultipleObjectsEx(emu),
        "WaitForSingleObjectEx" => kernel32::WaitForSingleObjectEx(emu),
        "WaitForThreadpoolIoCallbacks" => kernel32::WaitForThreadpoolIoCallbacks(emu),
        "WaitForThreadpoolTimerCallbacks" => kernel32::WaitForThreadpoolTimerCallbacks(emu),
        "WaitForThreadpoolWaitCallbacks" => kernel32::WaitForThreadpoolWaitCallbacks(emu),
        "WaitForThreadpoolWorkCallbacks" => kernel32::WaitForThreadpoolWorkCallbacks(emu),
        "WakeAllConditionVariable" => kernel32::WakeAllConditionVariable(emu),
        "WakeConditionVariable" => kernel32::WakeConditionVariable(emu),
        _ => return false,
    }
    true
}
