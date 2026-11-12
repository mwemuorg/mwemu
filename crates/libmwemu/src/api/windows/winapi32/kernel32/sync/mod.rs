use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

mod wait_named_pipe;
pub use wait_named_pipe::{WaitNamedPipeA, WaitNamedPipeW};

mod open_mutex;
pub use open_mutex::{OpenMutexA, OpenMutexW};

mod create_waitable_timer_ex;
pub use create_waitable_timer_ex::{CreateWaitableTimerExA, CreateWaitableTimerExW};

mod create_semaphore_ex;
pub use create_semaphore_ex::{CreateSemaphoreExA, CreateSemaphoreExW};

mod create_semaphore;
pub use create_semaphore::{CreateSemaphoreA, CreateSemaphoreW};

mod create_event;
pub use create_event::{CreateEventA, CreateEventW};

mod create_waitable_timer;
pub use create_waitable_timer::{CreateWaitableTimerA, CreateWaitableTimerW};

mod create_mutex;
pub use create_mutex::{CreateMutexA, CreateMutexW};

mod enter_critical_section;
pub use enter_critical_section::EnterCriticalSection;

mod fls_alloc;
pub use fls_alloc::FlsAlloc;

mod fls_get_value;
pub use fls_get_value::FlsGetValue;

mod fls_set_value;
pub use fls_set_value::FlsSetValue;

mod initialize_critical_section;
pub use initialize_critical_section::InitializeCriticalSection;

mod initialize_critical_section_and_spin_count;
pub use initialize_critical_section_and_spin_count::InitializeCriticalSectionAndSpinCount;

mod initialize_critical_section_ex;
pub use initialize_critical_section_ex::InitializeCriticalSectionEx;

mod interlocked_increment;
pub use interlocked_increment::InterlockedIncrement;

mod leave_critical_section;
pub use leave_critical_section::LeaveCriticalSection;

mod tls_alloc;
pub use tls_alloc::TlsAlloc;

mod tls_free;
pub use tls_free::TlsFree;

mod tls_get_value;
pub use tls_get_value::TlsGetValue;

mod tls_set_value;
pub use tls_set_value::TlsSetValue;

mod wait_for_single_object;
pub use wait_for_single_object::WaitForSingleObject;

mod cptp_quirks_init_once;
pub use cptp_quirks_init_once::CptpQuirksInitOnce;

mod init_once_get_string_table_offset;
pub use init_once_get_string_table_offset::InitOnceGetStringTableOffset;

mod initialize16_bit_critical_section;
pub use initialize16_bit_critical_section::Initialize16BitCriticalSection;

mod interlocked_compare_exchange;
pub use interlocked_compare_exchange::InterlockedCompareExchange;

mod interlocked_decrement;
pub use interlocked_decrement::InterlockedDecrement;

mod interlocked_exchange;
pub use interlocked_exchange::InterlockedExchange;

mod interlocked_exchange_add;
pub use interlocked_exchange_add::InterlockedExchangeAdd;

mod oobe_complete_wnf_wait_callback;
pub use oobe_complete_wnf_wait_callback::OOBECompleteWnfWaitCallback;

mod register_wait_for_input_idle;
pub use register_wait_for_input_idle::RegisterWaitForInputIdle;

mod register_wait_until_oobe_completed;
pub use register_wait_until_oobe_completed::RegisterWaitUntilOOBECompleted;

mod set_message_waiting_indicator;
pub use set_message_waiting_indicator::SetMessageWaitingIndicator;

mod trace_logging_register_ex__event_register__event_set_information;
pub use trace_logging_register_ex__event_register__event_set_information::TraceLoggingRegisterEx_EventRegister_EventSetInformation;

mod unregister_wait;
pub use unregister_wait::UnregisterWait;

mod unregister_wait_until_oobe_completed;
pub use unregister_wait_until_oobe_completed::UnregisterWaitUntilOOBECompleted;

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

mod reset_event;
pub use reset_event::ResetEvent;

mod wait_for_multiple_objects;
pub use wait_for_multiple_objects::WaitForMultipleObjects;

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
