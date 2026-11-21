pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    CompareStringOrdinal, DeleteCriticalSection, FlsFree, GetActiveProcessorCount,
    GetCurrentPackagePath, GetDateFormatA, GetDateFormatW, GetLocaleInfoEx,
    GetMaximumProcessorCount, GetProcessWorkingSetSize, GetStringTypeA, GetStringTypeW,
    GetTimeFormatA, GetTimeFormatW, GetUserDefaultUILanguage, GlobalDeleteAtom, GlobalFindAtomA,
    GlobalFindAtomW, GlobalFlags, GlobalGetAtomNameA, GlobalGetAtomNameW,
    InitializeConditionVariable, IsValidCodePage, LCMapStringA, LCMapStringW, SetHandleCount,
    SetLocaleInfoA, SetLocaleInfoW, SleepConditionVariableCS, SleepConditionVariableSRW, SleepEx,
    VerifyVersionInfoA, VerifyVersionInfoW,
};

use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::common::fallback::unsupported_exports;
use crate::winapi::common::wait::public_wait_handlers;
use crate::winapi::libraries::kernel32::dispatch_shared;
pub use crate::winapi::libraries::unsupported::kernel32::Beep;
use crate::winapi::winapi64::ntdll;

pub use crate::api::windows::common::threads::SuspendThread;
pub use crate::api::windows::libraries::kernel32::misc::*;

pub use crate::api::windows::libraries::kernel32::filesystem::{
    GetSystemWindowsDirectoryA, GetSystemWindowsDirectoryW,
};
pub use crate::api::windows::libraries::kernel32::process_thread::SetThreadContext;
pub use crate::api::windows::libraries::kernel32::strings_nls::{lstrcmpA, lstrcmpW};
pub use crate::api::windows::libraries::kernel32::system::GetOEMCP;
pub use crate::api::windows::libraries::kernel32::{
    CompareFileTime, DosDateTimeToFileTime, EtwEventEnabled, FileTimeToDosDateTime,
    FileTimeToLocalFileTime, FileTimeToSystemTime, LocalFileTimeToFileTime, SystemTimeToFileTime,
    VerSetConditionMask,
};

use crate::winapi::common::files::file_entry_points;

file_entry_points!(
    "kernel32",
    CopyFileExA,
    CopyFileExW,
    GetLongPathNameA,
    GetLongPathNameW,
    GetShortPathNameA,
    GetShortPathNameW,
    LockFile,
    MoveFileExA,
    MoveFileExW,
    SearchPathA,
    SearchPathW,
    UnlockFile,
);

mod console;
mod filesystem;
mod library_loader;
mod memory;
mod misc;
mod process_thread;
mod registry_profile;
mod strings_nls;
mod sync;
mod system;

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

mod appcompat;

mod diagnostics;

mod legacy_compat;

mod loader;

mod objects;

mod resolver;

mod time_date;

pub use loader::load_library;
pub(crate) use resolver::resolve_forwarded_export_string_depth;
pub use resolver::{
    dump_module_iat, guess_api_name, resolve_api_addr_to_name, resolve_api_name,
    resolve_api_name_in_module, search_api_name,
};
mod etw_event_register;
pub use etw_event_register::EtwEventRegister;
mod etw_event_set_information;
pub use etw_event_set_information::EtwEventSetInformation;
mod etw_event_write_transfer;
pub use etw_event_write_transfer::EtwEventWriteTransfer;
mod rtl_exit_user_process;
pub use rtl_exit_user_process::RtlExitUserProcess;

mod virtual_query;
pub use virtual_query::VirtualQuery;
mod virtual_query_ex;
pub use virtual_query_ex::VirtualQueryEx;
mod interlocked_increment;
pub use interlocked_increment::InterlockedIncrement;
mod interlocked_decrement;
pub use interlocked_decrement::InterlockedDecrement;
mod interlocked_exchange;
pub use interlocked_exchange::InterlockedExchange;
mod interlocked_compare_exchange;
pub use interlocked_compare_exchange::InterlockedCompareExchange;
mod wake_condition_variable;
pub use wake_condition_variable::WakeConditionVariable;
mod global_re_alloc;
pub use global_re_alloc::GlobalReAlloc;

public_wait_handlers!();

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    let api = api.rsplit('!').next().unwrap_or(&api);
    gateway_by_name(addr, api, emu)
}

pub(super) fn gateway_by_name(addr: u64, api: &str, emu: &mut emu::Emu) -> String {
    if dispatch_shared(api, "kernel32", emu) {
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

unsupported_exports!(
    "kernel32",
    SetProcessAffinityMask,
    SetProcessWorkingSetSize,
    SetThreadAffinityMask,
);
