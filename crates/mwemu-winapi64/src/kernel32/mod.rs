use crate::api::windows::common::fallback;
use crate::api::windows::common::memory as memory_api;
use crate::emu;
use crate::winapi::common::heap as heap_api;
use crate::winapi::common::memory_ops;
use crate::winapi::common::sections;
use crate::winapi::common::{files, handles, modules, strings};
use crate::winapi::winapi64::ntdll;

mod search_path;
pub use search_path::*;

mod open_mutex;
pub use open_mutex::*;

mod move_file_ex;
pub use move_file_ex::*;

mod global_get_atom_name;
pub use global_get_atom_name::*;

mod global_find_atom;
pub use global_find_atom::*;

mod get_system_windows_directory;
pub use get_system_windows_directory::*;

mod get_string_type;
pub use get_string_type::*;

mod get_short_path_name;
pub use get_short_path_name::*;

mod get_long_path_name;
pub use get_long_path_name::*;

mod appcompat;
mod console;
mod diagnostics;
mod filesystem;
mod legacy_compat;
mod library_loader;
mod memory;
mod misc;
mod objects;
mod process_thread;
mod registry_profile;
mod state;
mod strings_nls;
mod sync;
mod system;
mod time_date;

pub use crate::windows::loader64::*;
pub use console::*;
pub use filesystem::*;
pub use library_loader::*;
pub use memory::*;
pub use misc::*;
pub use process_thread::*;
pub use registry_profile::*;
pub use strings_nls::*;
pub use sync::*;
pub use system::*;

mod etw_event_enabled;
pub use etw_event_enabled::EtwEventEnabled;
mod etw_event_register;
pub use etw_event_register::EtwEventRegister;
mod etw_event_set_information;
pub use etw_event_set_information::EtwEventSetInformation;
mod etw_event_write_transfer;
pub use etw_event_write_transfer::EtwEventWriteTransfer;
mod rtl_exit_user_process;
pub use rtl_exit_user_process::RtlExitUserProcess;
mod fls_free;
pub use fls_free::FlsFree;
mod get_current_package_path;
pub use get_current_package_path::GetCurrentPackagePath;
mod is_valid_code_page;
pub use is_valid_code_page::IsValidCodePage;
mod lc_map_string_w;
pub use lc_map_string_w::LCMapStringW;
mod set_event;
pub use set_event::SetEvent;
mod set_handle_count;
pub use set_handle_count::SetHandleCount;
mod verify_version_info_a;
pub use verify_version_info_a::VerifyVersionInfoA;
mod ver_set_condition_mask;
pub use ver_set_condition_mask::VerSetConditionMask;
mod lstrcmp_a_api;
pub use lstrcmp_a_api::lstrcmpA_api;
mod lstrcmp_w_api;
pub use lstrcmp_w_api::lstrcmpW_api;
mod compare_string_ordinal;
pub use compare_string_ordinal::CompareStringOrdinal;
mod get_oemcp_2;
pub use get_oemcp_2::GetOEMCP;
mod virtual_query;
pub use virtual_query::VirtualQuery;
mod virtual_query_ex;
pub use virtual_query_ex::VirtualQueryEx;
mod copy_file_ex_w;
pub use copy_file_ex_w::CopyFileExW;
mod lock_file;
pub use lock_file::LockFile;
mod unlock_file;
pub use unlock_file::UnlockFile;
mod create_pipe;
pub use create_pipe::CreatePipe;
mod create_semaphore_a;
pub use create_semaphore_a::CreateSemaphoreA;
mod create_semaphore_ex_w;
pub use create_semaphore_ex_w::CreateSemaphoreExW;
mod create_waitable_timer_a;
pub use create_waitable_timer_a::CreateWaitableTimerA;
mod create_waitable_timer_ex_w;
pub use create_waitable_timer_ex_w::CreateWaitableTimerExW;
mod interlocked_increment;
pub use interlocked_increment::InterlockedIncrement;
mod interlocked_decrement;
pub use interlocked_decrement::InterlockedDecrement;
mod interlocked_exchange;
pub use interlocked_exchange::InterlockedExchange;
mod interlocked_compare_exchange;
pub use interlocked_compare_exchange::InterlockedCompareExchange;
mod delete_critical_section;
pub use delete_critical_section::DeleteCriticalSection;
mod initialize_condition_variable;
pub use initialize_condition_variable::InitializeConditionVariable;
mod wake_condition_variable;
pub use wake_condition_variable::WakeConditionVariable;
mod sleep_condition_variable_srw;
pub use sleep_condition_variable_srw::SleepConditionVariableSRW;
mod sleep_condition_variable_cs;
pub use sleep_condition_variable_cs::SleepConditionVariableCS;
mod sleep_ex;
pub use sleep_ex::SleepEx;
mod wait_for_single_object_ex;
pub use wait_for_single_object_ex::WaitForSingleObjectEx;
mod wait_for_multiple_objects_ex;
pub use wait_for_multiple_objects_ex::WaitForMultipleObjectsEx;
mod get_exit_code_process;
pub use get_exit_code_process::GetExitCodeProcess;
mod get_exit_code_thread;
pub use get_exit_code_thread::GetExitCodeThread;
mod get_process_id;
pub use get_process_id::GetProcessId;
mod set_thread_affinity_mask;
pub use set_thread_affinity_mask::SetThreadAffinityMask;
mod set_process_affinity_mask;
pub use set_process_affinity_mask::SetProcessAffinityMask;
mod get_active_processor_count;
pub use get_active_processor_count::GetActiveProcessorCount;
mod get_maximum_processor_count;
pub use get_maximum_processor_count::GetMaximumProcessorCount;
mod get_process_working_set_size;
pub use get_process_working_set_size::GetProcessWorkingSetSize;
mod set_process_working_set_size;
pub use set_process_working_set_size::SetProcessWorkingSetSize;
mod set_thread_context;
pub use set_thread_context::SetThreadContext;
mod suspend_thread;
pub use suspend_thread::SuspendThread;
mod process32_first_w;
pub use process32_first_w::Process32FirstW;
mod process32_next_w;
pub use process32_next_w::Process32NextW;
mod get_user_default_ui_language_2;
pub use get_user_default_ui_language_2::GetUserDefaultUILanguage;
mod set_locale_info_a;
pub use set_locale_info_a::SetLocaleInfoA;
mod get_locale_info_ex;
pub use get_locale_info_ex::GetLocaleInfoEx;
mod get_date_format_w;
pub use get_date_format_w::GetDateFormatW;
mod get_time_format_w;
pub use get_time_format_w::GetTimeFormatW;
mod global_delete_atom;
pub use global_delete_atom::GlobalDeleteAtom;
mod global_flags;
pub use global_flags::GlobalFlags;
mod global_size;
pub use global_size::GlobalSize;
mod global_re_alloc;
pub use global_re_alloc::GlobalReAlloc;
mod beep;
pub use beep::Beep;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if super::registry::gateway(api, emu) {
        return String::new();
    }
    if sections::dispatch(api, emu) {
        return String::new();
    }
    if memory_ops::dispatch(api, emu) {
        return String::new();
    }
    if heap_api::dispatch(api, emu) {
        return String::new();
    }
    if memory_api::dispatch(api, emu) {
        return String::new();
    }
    if modules::dispatch(api, "kernel32", emu)
        || files::dispatch(api, "kernel32", emu)
        || strings::dispatch(api, "kernel32", emu)
        || handles::dispatch(api, "kernel32", emu)
    {
        return String::new();
    }
    if appcompat::dispatch(api, addr, emu)
        || console::dispatch(api, addr, emu)
        || diagnostics::dispatch(api, addr, emu)
        || filesystem::dispatch(api, addr, emu)
        || library_loader::dispatch(api, addr, emu)
        || memory::dispatch(api, addr, emu)
        || objects::dispatch(api, addr, emu)
        || process_thread::dispatch(api, addr, emu)
        || registry_profile::dispatch(api, addr, emu)
        || strings_nls::dispatch(api, addr, emu)
        || sync::dispatch(api, addr, emu)
        || system::dispatch(api, addr, emu)
        || time_date::dispatch(api, addr, emu)
    {
        return String::new();
    }

    if ntdll::dispatch(addr, api, emu) {
        return String::new();
    }
    fallback::unimplemented(addr, api, emu)
}
