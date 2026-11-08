use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64::ntdll;

mod appcompat;
mod console;
mod filesystem;
mod legacy_compat;
mod library_loader;
mod loader;
mod memory;
mod misc;
mod process_thread;
mod registry_profile;
mod resolver;
mod state;
mod strings_nls;
mod sync;
mod system;

pub use console::*;
pub use filesystem::*;
pub use library_loader::*;
pub use loader::*;
pub use memory::*;
pub use misc::*;
pub use process_thread::*;
pub use registry_profile::*;
pub use resolver::*;
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
mod get_string_type_w_api;
pub use get_string_type_w_api::GetStringTypeW_api;
mod is_valid_code_page_api;
pub use is_valid_code_page_api::IsValidCodePage_api;
mod lc_map_string_w_api;
pub use lc_map_string_w_api::LCMapStringW_api;
mod set_event;
pub use set_event::SetEvent;
mod set_handle_count_api;
pub use set_handle_count_api::SetHandleCount_api;
mod verify_version_info_a_api;
pub use verify_version_info_a_api::VerifyVersionInfoA_api;
mod ver_set_condition_mask;
pub use ver_set_condition_mask::VerSetConditionMask;
mod lstrcmp_a_api;
pub use lstrcmp_a_api::lstrcmpA_api;
mod lstrcmp_w_api;
pub use lstrcmp_w_api::lstrcmpW_api;
mod compare_string_a;
pub use compare_string_a::CompareStringA;
mod compare_string_ordinal;
pub use compare_string_ordinal::CompareStringOrdinal;
mod get_string_type_a;
pub use get_string_type_a::GetStringTypeA;
mod get_oemcp_2;
pub use get_oemcp_2::GetOEMCP;
mod virtual_query_api;
pub use virtual_query_api::VirtualQuery_api;
mod virtual_query_ex_api;
pub use virtual_query_ex_api::VirtualQueryEx_api;
mod get_long_path_name_a_api;
pub use get_long_path_name_a_api::GetLongPathNameA_api;
mod get_long_path_name_w_api;
pub use get_long_path_name_w_api::GetLongPathNameW_api;
mod get_short_path_name_a_api;
pub use get_short_path_name_a_api::GetShortPathNameA_api;
mod get_short_path_name_w_api;
pub use get_short_path_name_w_api::GetShortPathNameW_api;
mod search_path_a;
pub use search_path_a::SearchPathA;
mod search_path_w;
pub use search_path_w::SearchPathW;
mod copy_file_ex_w;
pub use copy_file_ex_w::CopyFileExW;
mod move_file_ex_a_api;
pub use move_file_ex_a_api::MoveFileExA_api;
mod move_file_ex_w;
pub use move_file_ex_w::MoveFileExW;
mod flush_file_buffers;
pub use flush_file_buffers::FlushFileBuffers;
mod set_end_of_file;
pub use set_end_of_file::SetEndOfFile;
mod lock_file;
pub use lock_file::LockFile;
mod unlock_file;
pub use unlock_file::UnlockFile;
mod create_pipe;
pub use create_pipe::CreatePipe;
mod create_semaphore_a_api;
pub use create_semaphore_a_api::CreateSemaphoreA_api;
mod create_semaphore_ex_w;
pub use create_semaphore_ex_w::CreateSemaphoreExW;
mod create_waitable_timer_a_api;
pub use create_waitable_timer_a_api::CreateWaitableTimerA_api;
mod create_waitable_timer_ex_w;
pub use create_waitable_timer_ex_w::CreateWaitableTimerExW;
mod open_mutex_a_api;
pub use open_mutex_a_api::OpenMutexA_api;
mod open_mutex_w;
pub use open_mutex_w::OpenMutexW;
mod interlocked_increment_api;
pub use interlocked_increment_api::InterlockedIncrement_api;
mod interlocked_decrement_api;
pub use interlocked_decrement_api::InterlockedDecrement_api;
mod interlocked_exchange_api;
pub use interlocked_exchange_api::InterlockedExchange_api;
mod interlocked_compare_exchange_api;
pub use interlocked_compare_exchange_api::InterlockedCompareExchange_api;
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
mod set_thread_affinity_mask_api;
pub use set_thread_affinity_mask_api::SetThreadAffinityMask_api;
mod set_process_affinity_mask_api;
pub use set_process_affinity_mask_api::SetProcessAffinityMask_api;
mod get_active_processor_count_api;
pub use get_active_processor_count_api::GetActiveProcessorCount_api;
mod get_maximum_processor_count_api;
pub use get_maximum_processor_count_api::GetMaximumProcessorCount_api;
mod get_process_working_set_size_api;
pub use get_process_working_set_size_api::GetProcessWorkingSetSize_api;
mod set_process_working_set_size_api;
pub use set_process_working_set_size_api::SetProcessWorkingSetSize_api;
mod set_thread_context_api;
pub use set_thread_context_api::SetThreadContext_api;
mod suspend_thread;
pub use suspend_thread::SuspendThread;
mod process32_first_w_api;
pub use process32_first_w_api::Process32FirstW_api;
mod process32_next_w_api;
pub use process32_next_w_api::Process32NextW_api;
mod get_user_default_ui_language_2;
pub use get_user_default_ui_language_2::GetUserDefaultUILanguage;
mod set_locale_info_a_api;
pub use set_locale_info_a_api::SetLocaleInfoA_api;
mod get_locale_info_ex;
pub use get_locale_info_ex::GetLocaleInfoEx;
mod get_date_format_w;
pub use get_date_format_w::GetDateFormatW;
mod get_time_format_w;
pub use get_time_format_w::GetTimeFormatW;
mod global_add_atom_w_api;
pub use global_add_atom_w_api::GlobalAddAtomW_api;
mod global_delete_atom_api;
pub use global_delete_atom_api::GlobalDeleteAtom_api;
mod global_find_atom_a_api;
pub use global_find_atom_a_api::GlobalFindAtomA_api;
mod global_find_atom_w_api;
pub use global_find_atom_w_api::GlobalFindAtomW_api;
mod global_get_atom_name_a_api;
pub use global_get_atom_name_a_api::GlobalGetAtomNameA_api;
mod global_get_atom_name_w_api;
pub use global_get_atom_name_w_api::GlobalGetAtomNameW_api;
mod global_flags_api;
pub use global_flags_api::GlobalFlags_api;
mod global_size_api;
pub use global_size_api::GlobalSize_api;
mod global_re_alloc_api;
pub use global_re_alloc_api::GlobalReAlloc_api;
mod beep;
pub use beep::Beep;
mod get_system_windows_directory_a_api;
pub use get_system_windows_directory_a_api::GetSystemWindowsDirectoryA_api;
mod get_system_windows_directory_w_api;
pub use get_system_windows_directory_w_api::GetSystemWindowsDirectoryW_api;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if super::registry::gateway(api, emu) {
        return String::new();
    }
    match api {
        "AcquireSRWLockExclusive" => AcquireSRWLockExclusive(emu),
        "AcquireSRWLockShared" => AcquireSRWLockShared(emu),
        "ActivateActCtx" => ActivateActCtx(emu),
        "AddVectoredExceptionHandler" => AddVectoredExceptionHandler(emu),
        "AreFileApiIsAnsi" | "AreFileApisANSI" => AreFileApiIsAnsi(emu),
        "BeginUpdateResourceA" => BeginUpdateResourceA(emu),
        "CloseHandle" => CloseHandle(emu),
        "CompareStringW" => CompareStringW(emu),
        "ConnectNamedPipe" => ConnectNamedPipe(emu),
        "DeleteFileA" => DeleteFileA(emu),
        "DeleteFileW" => DeleteFileW(emu),
        "DuplicateHandle" => DuplicateHandle(emu),
        "CopyFileA" => CopyFileA(emu),
        "CopyFileW" => CopyFileW(emu),
        "CreateActCtxA" => CreateActCtxA(emu),
        "CreateDirectoryA" => CreateDirectoryA(emu),
        "CreateDirectoryW" => CreateDirectoryW(emu),
        "CreateEventA" => CreateEventA(emu),
        "CreateEventW" => CreateEventW(emu),
        "CreateFileA" => CreateFileA(emu),
        "CreateFileW" => CreateFileW(emu),
        "CreateFileMappingA" => CreateFileMappingA(emu),
        "CreateFileMappingW" => CreateFileMappingW(emu),
        "CreateMutexA" => CreateMutexA(emu),
        "CreateMutexW" => CreateMutexW(emu),
        "CreateNamedPipeA" => CreateNamedPipeA(emu),
        "CreateNamedPipeW" => CreateNamedPipeW(emu),
        "CreateProcessA" => CreateProcessA(emu),
        "CreateProcessW" => CreateProcessW(emu),
        "CreateRemoteThread" => CreateRemoteThread(emu),
        "CreateThread" => CreateThread(emu),
        "CreateToolhelp32Snapshot" => CreateToolhelp32Snapshot(emu),
        "DecodePointer" => DecodePointer(emu),
        "DisconnectNamedPipe" => DisconnectNamedPipe(emu),
        "EncodePointer" => EncodePointer(emu),
        "EnterCriticalSection" => EnterCriticalSection(emu),
        "EtwEventEnabled" | "EventEnabled" => EtwEventEnabled(emu),
        "EtwEventRegister" | "EventRegister" => EtwEventRegister(emu),
        "EtwEventSetInformation" | "EventSetInformation" => EtwEventSetInformation(emu),
        "EtwEventWriteTransfer" | "EventWriteTransfer" => EtwEventWriteTransfer(emu),
        "ExitProcess" => ExitProcess(emu),
        "ExitThread" => ExitThread(emu),
        "RtlExitUserProcess" => RtlExitUserProcess(emu),
        "ExpandEnvironmentStringsA" => ExpandEnvironmentStringsA(emu),
        "ExpandEnvironmentStringsW" => ExpandEnvironmentStringsW(emu),
        "FileTimeToSystemTime" => FileTimeToSystemTime(emu),
        "FindActCtxSectionStringW" => FindActCtxSectionStringW(emu),
        "FindClose" => FindClose(emu),
        "FindFirstFileA" => FindFirstFileA(emu),
        "FindFirstFileExW" => FindFirstFileExW(emu),
        "FindFirstFileW" => FindFirstFileW(emu),
        "FindNextFileA" => FindNextFileA(emu),
        "FindNextFileW" => FindNextFileW(emu),
        "FindResourceA" => FindResourceA(emu),
        "FindResourceW" => FindResourceW(emu),
        "FormatMessageA" => FormatMessageA(emu),
        "FormatMessageW" => FormatMessageW(emu),
        "FreeLibrary" => FreeLibrary(emu),
        "FlsAlloc" => FlsAlloc(emu),
        "FlsFree" => FlsFree(emu),
        "FlsGetValue" => FlsGetValue(emu),
        "FlsGetValue2" => FlsGetValue2(emu),
        "FlsSetValue" => FlsSetValue(emu),
        "FreeResource" => FreeResource(emu),
        "GetACP" => GetACP(emu),
        "GetCommandLineA" => GetCommandLineA(emu),
        "GetCommandLineW" => GetCommandLineW(emu),
        "GetComputerNameA" => GetComputerNameA(emu),
        "GetComputerNameW" => GetComputerNameW(emu),
        "GetDiskFreeSpaceA" => GetDiskFreeSpaceA(emu),
        "GetDiskFreeSpaceExA" => GetDiskFreeSpaceExA(emu),
        "GetDiskFreeSpaceExW" => GetDiskFreeSpaceExW(emu),
        "GetDriveTypeA" => GetDriveTypeA(emu),
        "GetDriveTypeW" => GetDriveTypeW(emu),
        "GetConsoleCP" => GetConsoleCP(emu),
        "GetConsoleMode" => GetConsoleMode(emu),
        "GetConsoleOutputCP" => GetConsoleOutputCP(emu),
        "GetCPInfo" => GetCPInfo(emu),
        "GetCurrentDirectoryA" => GetCurrentDirectoryA(emu),
        "GetCurrentDirectoryW" => GetCurrentDirectoryW(emu),
        "GetCurrentProcess" => GetCurrentProcess(emu),
        "GetCurrentProcessId" => GetCurrentProcessId(emu),
        "GetCurrentThread" => GetCurrentThread(emu),
        "GetCurrentThreadId" => GetCurrentThreadId(emu),
        "GetEnvironmentStringsW" => GetEnvironmentStringsW(emu),
        "FreeEnvironmentStringsA" => FreeEnvironmentStrings(emu),
        "FreeEnvironmentStringsW" => FreeEnvironmentStrings(emu),
        "GetEnvironmentVariableA" => GetEnvironmentVariableA(emu),
        "GetEnvironmentVariableW" => GetEnvironmentVariableW(emu),
        "GetFileAttributesA" => GetFileAttributesA(emu),
        "GetFileAttributesW" => GetFileAttributesW(emu),
        "GetFileSize" => GetFileSize(emu),
        "GetFileTime" => GetFileTime(emu),
        "SetFileTime" => SetFileTime(emu),
        "GetFileSizeEx" => GetFileSizeEx(emu),
        "GetFileType" => GetFileType(emu),
        "GetFullPathNameA" => GetFullPathNameA(emu),
        "GetFullPathNameW" => GetFullPathNameW(emu),
        "GetCurrentPackagePath" => GetCurrentPackagePath(emu),
        "GetLastError" => GetLastError(emu),
        "GetLocaleInfoA" => GetLocaleInfoA(emu),
        "GetLocaleInfoW" => GetLocaleInfoW(emu),
        "GetLocalTime" => GetLocalTime(emu),
        "GetLogicalDrives" => GetLogicalDrives(emu),
        "GetModuleFileNameA" => GetModuleFileNameA(emu),
        "GetModuleFileNameW" => GetModuleFileNameW(emu),
        "GetModuleHandleA" => GetModuleHandleA(emu),
        "GetModuleHandleExW" => GetModuleHandleExW(emu),
        "GetModuleHandleW" => GetModuleHandleW(emu),
        "GetNativeSystemInfo" => GetNativeSystemInfo(emu),
        "GetProcAddress" => GetProcAddress(emu),
        "GetProcessAffinityMask" => GetProcessAffinityMask(emu),
        "GetProcessHeap" => GetProcessHeap(emu),
        "GetStartupInfoA" => GetStartupInfoA(emu),
        "GetStartupInfoW" => GetStartupInfoW(emu),
        "GetStdHandle" => GetStdHandle(emu),
        "GetStringTypeW" => GetStringTypeW_api(emu),
        "GetThreadId" => GetThreadId(emu),
        "GetSystemDirectoryA" => GetSystemDirectoryA(emu),
        "GetSystemDirectoryW" => GetSystemDirectoryW(emu),
        "GetSystemFirmwareTable" => GetSystemFirmwareTable(emu),
        "GetSystemInfo" => GetSystemInfo(emu),
        "GetSystemTime" => GetSystemTime(emu),
        "GetSystemTimeAsFileTime" => GetSystemTimeAsFileTime(emu),
        "GetSystemTimePreciseAsFileTime" => GetSystemTimePreciseAsFileTime(emu),
        "GetTempPathA" => GetTempPathA(emu),
        "GetTempPathW" => GetTempPathW(emu),
        "GetThreadContext" => GetThreadContext(emu),
        "GetThreadLocale" => GetThreadLocale(emu),
        "GetTickCount" => GetTickCount(emu),
        "GetTickCount64" => GetTickCount64(emu),
        "GetTimeZoneInformation" => GetTimeZoneInformation(emu),
        "GetTimeZoneInformationForYear" => GetTimeZoneInformationForYear(emu),
        "GetUserDefaultLangId" => GetUserDefaultLangId(emu),
        "GetUserDefaultLCID" => GetUserDefaultLCID(emu),
        "GetVersion" => GetVersion(emu),
        "GetVersionExA" => GetVersionExA(emu),
        "GetVersionExW" => GetVersionExW(emu),
        "GetWindowsDirectoryA" => GetWindowsDirectoryA(emu),
        "GetWindowsDirectoryW" => GetWindowsDirectoryW(emu),
        "GlobalAddAtomA" => GlobalAddAtomA(emu),
        "GlobalAlloc" => GlobalAlloc(emu),
        "GlobalFree" => GlobalFree(emu),
        "GlobalLock" => GlobalLock(emu),
        "GlobalUnlock" => GlobalUnlock(emu),
        "HeapAlloc" => HeapAlloc(emu),
        "HeapCreate" => HeapCreate(emu),
        "HeapDestroy" => HeapDestroy(emu),
        "HeapSetInformation" => HeapSetInformation(emu),
        "HeapFree" => HeapFree(emu),
        "HeapReAlloc" => HeapReAlloc(emu),
        "HeapSize" => HeapSize(emu),
        "InitializeCriticalSection" => InitializeCriticalSection(emu),
        "InitializeCriticalSectionAndSpinCount" => InitializeCriticalSectionAndSpinCount(emu),
        "InitializeCriticalSectionEx" => InitializeCriticalSectionEx(emu),
        "InitializeSRWLock" => InitializeSRWLock(emu),
        "InitOnceBeginInitialize" => InitOnceBeginInitialize(emu),
        "IsBadReadPtr" => IsBadReadPtr(emu),
        "CheckRemoteDebuggerPresent" => CheckRemoteDebuggerPresent(emu),
        "IsDebuggerPresent" => IsDebuggerPresent(emu),
        "QueryDosDeviceW" => QueryDosDeviceW(emu),
        "IsValidCodePage" => IsValidCodePage_api(emu),
        "IsWow64Process" => IsWow64Process(emu),
        "IsProcessorFeaturePresent" => IsProcessorFeaturePresent(emu),
        "LeaveCriticalSection" => LeaveCriticalSection(emu),
        "Module32First" => Module32First(emu),
        "Module32FirstW" => Module32FirstW(emu),
        "Module32Next" => Module32Next(emu),
        "Module32NextW" => Module32NextW(emu),
        "LoadLibraryA" => LoadLibraryA(emu),
        "LoadLibraryExA" => LoadLibraryExA(emu),
        "LoadLibraryExW" => LoadLibraryExW(emu),
        "LoadLibraryW" => LoadLibraryW(emu),
        "LoadResource" => LoadResource(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "LocalFree" => LocalFree(emu),
        "LockResource" => LockResource(emu),
        "LCMapStringW" => LCMapStringW_api(emu),
        "lstrcatA" => lstrcatA(emu),
        "lstrcatW" => lstrcatW(emu),
        "LStrCmpI" => LStrCmpI(emu),
        "lstrcmpiW" => LStrCmpIW(emu),
        "LStrCmpIW" => LStrCmpIW(emu),
        "lstrcpy" => lstrcpy(emu),
        "lstrcpyn" => lstrcpyn(emu),
        "lstrcpyW" => lstrcpyW(emu),
        "lstrlen" => lstrlenA(emu),
        "lstrlenA" => lstrlenA(emu),
        "lstrlenW" => lstrlenW(emu),
        "MapViewOfFile" => MapViewOfFile(emu),
        "MoveFileA" => MoveFileA(emu),
        "MoveFileW" => MoveFileW(emu),
        "MultiByteToWideChar" => MultiByteToWideChar(emu),
        "OutputDebugStringA" => OutputDebugStringA(emu),
        "OutputDebugStringW" => OutputDebugStringW(emu),
        "OpenProcess" => OpenProcess(emu),
        "OpenProcessToken" => OpenProcessToken(emu),
        "OpenThread" => OpenThread(emu),

        "Process32First" => Process32First(emu),
        "Process32Next" => Process32Next(emu),
        "QueryPerformanceCounter" => QueryPerformanceCounter(emu),
        "QueryPerformanceFrequency" => QueryPerformanceFrequency(emu),
        "RaiseException" => RaiseException(emu),
        "ReadFile" => ReadFile(emu),
        "ReadProcessMemory" => ReadProcessMemory(emu),
        "ResetEvent" => ResetEvent(emu),
        "SetEvent" => SetEvent(emu),

        "ReleaseSRWLockExclusive" => ReleaseSRWLockExclusive(emu),
        "ReleaseSRWLockShared" => ReleaseSRWLockShared(emu),
        "ResumeThread" => ResumeThread(emu),
        "SetCurrentDirectoryA" => SetCurrentDirectoryA(emu),
        "SetEnvironmentVariableA" => SetEnvironmentVariableA(emu),
        "SetEnvironmentVariableW" => SetEnvironmentVariableW(emu),
        "SetErrorMode" => SetErrorMode(emu),
        "SetHandleCount" => SetHandleCount_api(emu),
        "SetFilePointer" => SetFilePointer(emu),
        "SetLastError" => SetLastError(emu),
        "SetThreadLocale" => SetThreadLocale(emu),
        "SetThreadStackGuarantee" => SetThreadStackGuarantee(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "SizeofResource" => SizeofResource(emu),
        "Sleep" => Sleep(emu),
        "SystemTimeToFileTime" => SystemTimeToFileTime(emu),
        "SystemTimeToTzSpecificLocalTime" => SystemTimeToTzSpecificLocalTime(emu),
        "TerminateProcess" => TerminateProcess(emu),
        "Thread32First" => Thread32First(emu),
        "Thread32Next" => Thread32Next(emu),
        "TryAcquireSRWLockExclusive" => TryAcquireSRWLockExclusive(emu),
        "TryAcquireSRWLockShared" => TryAcquireSRWLockShared(emu),
        "TlsAlloc" => TlsAlloc(emu),
        "TlsFree" => TlsFree(emu),
        "TlsGetValue" => TlsGetValue(emu),
        "TlsGetValue2" => TlsGetValue2(emu),
        "TlsSetValue" => TlsSetValue(emu),
        "UnhandledExceptionFilter" => UnhandledExceptionFilter(emu),
        "VirtualAlloc" => VirtualAlloc(emu),
        "VirtualAllocEx" => VirtualAllocEx(emu),
        "VirtualAllocExNuma" => VirtualAllocExNuma(emu),
        "VirtualFree" => VirtualFree(emu),
        "VirtualLock" => VirtualLock(emu),
        "VirtualProtect" => VirtualProtect(emu),
        "VirtualProtectEx" => VirtualProtectEx(emu),
        "WaitForMultipleObjects" => WaitForMultipleObjects(emu),
        "WaitForSingleObject" => WaitForSingleObject(emu),
        "VerifyVersionInfoA" | "VerifyVersionInfoW" => VerifyVersionInfoA_api(emu, api),
        "VerSetConditionMask" => VerSetConditionMask(emu),
        "WideCharToMultiByte" => WideCharToMultiByte(emu),
        "WinExec" => WinExec(emu),
        "WriteConsoleW" => WriteConsoleW(emu),
        "WriteFile" => WriteFile(emu),
        "WriteProcessMemory" => WriteProcessMemory(emu),
        "GetVolumeInformationA" => GetVolumeInformationA(emu),
        "GetVolumeInformationW" => GetVolumeInformationW(emu),
        "GetVolumePathNameA" => GetVolumePathNameA(emu),
        "GetVolumePathNameW" => GetVolumePathNameW(emu),
        "RemoveDirectoryA" => RemoveDirectoryA(emu),
        "RemoveDirectoryW" => RemoveDirectoryW(emu),
        "SetFileAttributesA" => SetFileAttributesA(emu),
        "SetFileAttributesW" => SetFileAttributesW(emu),
        "DeviceIoControl" => api_DeviceIoControl(emu),

        // --- String/Comparison ---
        "lstrcmpA" => lstrcmpA_api(emu),
        "lstrcmpW" => lstrcmpW_api(emu),
        "CompareStringA" => CompareStringA(emu),
        "CompareStringOrdinal" => CompareStringOrdinal(emu),
        "GetStringTypeA" => GetStringTypeA(emu),
        "GetOEMCP" => GetOEMCP(emu),

        // --- Virtual Memory ---
        "VirtualQuery" => VirtualQuery_api(emu, addr),
        "VirtualQueryEx" => VirtualQueryEx_api(emu, addr),

        // --- Path/File ---
        "GetLongPathNameA" => GetLongPathNameA_api(emu),
        "GetLongPathNameW" => GetLongPathNameW_api(emu),
        "GetShortPathNameA" => GetShortPathNameA_api(emu),
        "GetShortPathNameW" => GetShortPathNameW_api(emu),
        "SearchPathA" => SearchPathA(emu),
        "SearchPathW" => SearchPathW(emu),
        "CopyFileExW" => CopyFileExW(emu),
        "MoveFileExA" => MoveFileExA_api(emu),
        "MoveFileExW" => MoveFileExW(emu),
        "SetFilePointerEx" => SetFilePointerEx(emu),
        "FlushFileBuffers" => FlushFileBuffers(emu),
        "SetEndOfFile" => SetEndOfFile(emu),
        "LockFile" => LockFile(emu),
        "UnlockFile" => UnlockFile(emu),
        "CreatePipe" => CreatePipe(emu),
        "GetPrivateProfileStringA" => GetPrivateProfileStringA(emu),
        "GetPrivateProfileStringW" => GetPrivateProfileStringW(emu),
        "GetProfileStringA" => GetProfileStringA(emu),
        "GetProfileStringW" => GetProfileStringW(emu),
        "WritePrivateProfileStringA" => WritePrivateProfileStringA(emu),
        "WritePrivateProfileStringW" => WritePrivateProfileStringW(emu),
        "WriteProfileStringA" => WriteProfileStringA(emu),
        "WriteProfileStringW" => WriteProfileStringW(emu),
        "GetPrivateProfileIntA" => GetPrivateProfileIntA(emu),
        "GetPrivateProfileIntW" => GetPrivateProfileIntW(emu),
        "GetProfileIntA" => GetProfileIntA(emu),
        "GetProfileIntW" => GetProfileIntW(emu),

        // --- Synchronization ---
        "CreateSemaphoreA" | "CreateSemaphoreW" => CreateSemaphoreA_api(emu, api),
        "CreateSemaphoreExW" => CreateSemaphoreExW(emu),
        "CreateWaitableTimerA" | "CreateWaitableTimerW" => CreateWaitableTimerA_api(emu, api),
        "CreateWaitableTimerExW" => CreateWaitableTimerExW(emu),
        "OpenMutexA" => OpenMutexA_api(emu),
        "OpenMutexW" => OpenMutexW(emu),
        "InterlockedIncrement" => InterlockedIncrement_api(emu, addr),
        "InterlockedDecrement" => InterlockedDecrement_api(emu, addr),
        "InterlockedExchange" => InterlockedExchange_api(emu, addr),
        "InterlockedCompareExchange" => InterlockedCompareExchange_api(emu, addr),
        "DeleteCriticalSection" => DeleteCriticalSection(emu),
        "InitializeConditionVariable" => InitializeConditionVariable(emu),
        "WakeConditionVariable" | "WakeAllConditionVariable" => WakeConditionVariable(emu, api),
        "SleepConditionVariableSRW" => SleepConditionVariableSRW(emu),
        "SleepConditionVariableCS" => SleepConditionVariableCS(emu),
        "SleepEx" => SleepEx(emu),
        "WaitForSingleObjectEx" => WaitForSingleObjectEx(emu),
        "WaitForMultipleObjectsEx" => WaitForMultipleObjectsEx(emu),

        // --- Process/Thread ---
        "GetExitCodeProcess" => GetExitCodeProcess(emu),
        "GetExitCodeThread" => GetExitCodeThread(emu),
        "GetProcessId" => GetProcessId(emu),
        "SetThreadAffinityMask" => SetThreadAffinityMask_api(emu),
        "SetProcessAffinityMask" => SetProcessAffinityMask_api(emu),
        "GetActiveProcessorCount" => GetActiveProcessorCount_api(emu),
        "GetMaximumProcessorCount" => GetMaximumProcessorCount_api(emu),
        "GetProcessWorkingSetSize" => GetProcessWorkingSetSize_api(emu),
        "SetProcessWorkingSetSize" => SetProcessWorkingSetSize_api(emu),
        "SetThreadContext" => SetThreadContext_api(emu),
        "SuspendThread" => SuspendThread(emu),
        "Process32FirstW" => Process32FirstW_api(emu),
        "Process32NextW" => Process32NextW_api(emu),

        // --- Locale/NLS ---
        "GetUserDefaultUILanguage" => GetUserDefaultUILanguage(emu),
        "SetLocaleInfoA" | "SetLocaleInfoW" => SetLocaleInfoA_api(emu, api),
        "GetLocaleInfoEx" => GetLocaleInfoEx(emu),
        "GetDateFormatW" => GetDateFormatW(emu),
        "GetTimeFormatW" => GetTimeFormatW(emu),

        // --- Atom table ---
        "GlobalAddAtomW" => GlobalAddAtomW_api(emu),
        "GlobalDeleteAtom" => GlobalDeleteAtom_api(emu),
        "GlobalFindAtomA" => GlobalFindAtomA_api(emu),
        "GlobalFindAtomW" => GlobalFindAtomW_api(emu),
        "GlobalGetAtomNameA" => GlobalGetAtomNameA_api(emu),
        "GlobalGetAtomNameW" => GlobalGetAtomNameW_api(emu),
        "GlobalFlags" => GlobalFlags_api(emu),
        "GlobalSize" => GlobalSize_api(emu),
        "GlobalReAlloc" => GlobalReAlloc_api(emu, addr),

        // --- Environment ---
        "GetEnvironmentStrings" | "GetEnvironmentStringsA" => GetEnvironmentStringsA(emu),

        // --- Misc ---
        "Beep" => Beep(emu),
        "GetSystemWindowsDirectoryA" => GetSystemWindowsDirectoryA_api(emu),
        "GetSystemWindowsDirectoryW" => GetSystemWindowsDirectoryW_api(emu),
        _ => {
            // Windows forwards many Rtl*/Nt* functions through kernel32 →
            // ntdll. Try ntdll sub-dispatchers before giving up.
            if ntdll::dispatch(addr, api, emu) {
                return String::new();
            }

            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}
