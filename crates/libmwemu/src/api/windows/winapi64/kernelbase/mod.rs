pub use crate::winapi::libraries::kernel32::entry_points::kernelbase::{
    __C_specific_handler, __chkstk, __misaligned_access, __wargv, _local_unwind,
    AcquireSRWLockExclusive, AcquireSRWLockShared, CloseThreadpoolTimer, CloseThreadpoolWait,
    CompareStringA, CompareStringOrdinal, CompareStringW, CreateActCtxW, CreateEnclave,
    CreateIoRing, CreateProcessA, CreateProcessW, CreateRemoteThread, CreateRemoteThreadEx,
    DeleteCriticalSection, DuplicateHandle, EnterCriticalSection, FlsFree,
    FlushProcessWriteBuffers, FormatMessageA, FormatMessageW, FreeLibrary, GetCPInfo, GetConsoleCP,
    GetCurrentProcessorNumber, GetLastError, GetLocaleInfoA, GetLocaleInfoEx, GetLocaleInfoW,
    GetNativeSystemInfo, GetProcessWorkingSetSize, GetStartupInfoW, GetStringTypeA, GetStringTypeW,
    GetSystemInfo, GetThreadLocale, GetUserDefaultLCID, GlobalFlags, GlobalLock, GlobalUnlock,
    InitOnceBeginInitialize, InitOnceExecuteOnce, InitOnceInitialize, InitializeConditionVariable,
    InitializeCriticalSection, InitializeCriticalSectionEx, InitializeSRWLock,
    InterlockedDecrement, InterlockedFlushSList, InterlockedIncrement, InterlockedPopEntrySList,
    InterlockedPushListSList, IsDebuggerPresent, IsEnclaveTypeSupported, IsProcessorFeaturePresent,
    IsThreadpoolTimerSet, IsUserCetAvailableInEnvironment, IsValidCodePage, IsWow64Process,
    LCMapStringA, LCMapStringW, LeaveCriticalSection, LoadLibraryA, LoadLibraryExA, LoadLibraryExW,
    LoadLibraryW, LoadStringA, LoadStringW, MultiByteToWideChar, NotifyFeatureToggleUsage,
    OpenPackageInfoByFullName, OpenState, OpenStateExplicit, OpenThreadToken,
    ReleaseMutexWhenCallbackReturns, ReleaseSRWLockExclusive, ReleaseSRWLockShared,
    ReleaseSemaphoreWhenCallbackReturns, SetLastConsoleEventActive, SetLastError,
    SetThreadpoolTimer, SetThreadpoolTimerEx, SetThreadpoolWait, SetThreadpoolWaitEx, Sleep,
    SleepConditionVariableCS, SleepEx, TryAcquireSRWLockExclusive, TryAcquireSRWLockShared,
    TryEnterCriticalSection, UnhandledExceptionFilter, UnmapViewOfFile, WaitForDebugEventEx,
    WaitForThreadpoolIoCallbacks, WaitForThreadpoolTimerCallbacks, WaitForThreadpoolWaitCallbacks,
    WaitForThreadpoolWorkCallbacks, WakeAllConditionVariable, WakeConditionVariable,
};

pub use crate::api::windows::libraries::advapi32::{
    AreAllAccessesGranted, AreAnyAccessesGranted, GetAce, GetAclInformation, InitializeAcl,
    MapGenericMask,
};
pub use crate::api::windows::registry::{
    RegCloseKey, RegCreateKeyExA, RegCreateKeyExW, RegDeleteKeyExA, RegDeleteKeyExW,
    RegDeleteTreeA, RegDeleteTreeW, RegDeleteValueA, RegDeleteValueW, RegEnumKeyExA, RegEnumKeyExW,
    RegEnumValueA, RegEnumValueW, RegFlushKey, RegGetValueA, RegGetValueW, RegOpenKeyExA,
    RegOpenKeyExW, RegQueryInfoKeyA, RegQueryInfoKeyW, RegQueryValueExA, RegQueryValueExW,
    RegSetValueExA, RegSetValueExW,
};
use crate::emu;
use crate::winapi::common::fallback::unsupported_exports;
use crate::winapi::libraries::kernel32::dispatch_shared;
pub use crate::winapi::libraries::unsupported::kernelbase::{
    ActivateActCtx, AddDllDirectory, AddVectoredContinueHandler, AppPolicyGetClrCompat,
    AppPolicyGetCreateFileAccess, AppPolicyGetLifecycleManagement,
    AppPolicyGetMediaFoundationCodecLoading, AppPolicyGetShowDeveloperDiagnostic,
    AppPolicyGetThreadInitializationType, AppPolicyGetWindowingModel, AppXGetOSMaxVersionTested,
    Beep, BuildIoRingCancelRequest, BuildIoRingFlushFile, BuildIoRingReadFile,
    BuildIoRingReadFileScatter, BuildIoRingRegisterBuffers, BuildIoRingRegisterFileHandles,
    BuildIoRingWriteFile, BuildIoRingWriteFileGather, CancelThreadpoolIo, CeipIsOptedIn,
    CheckRemoteDebuggerPresent, CopyMemoryNonTemporal, CtrlRoutine, DecodeRemotePointer,
    DecodeSystemPointer, DeviceIoControl, DisassociateCurrentThreadFromCallback,
    DiscardVirtualMemory, EncodeRemotePointer, EncodeSystemPointer, EventActivityIdControl,
    EventProviderEnabled, EventWrite, EventWriteEx, EventWriteString, FindActCtxSectionStringW,
    FindPackagesByPackageFamily, FlsAlloc, FlsGetValue, FlsGetValue2, FlsSetValue,
    FormatApplicationUserModelId, GetApplicationUserModelId, GetConsoleInputExeNameA,
    GetConsoleInputExeNameW, GetConsoleOutputCP, GetCurrentApplicationUserModelId,
    GetCurrentPackageFamilyName, GetCurrentPackageFullName, GetCurrentPackageId,
    GetCurrentPackageInfo, GetCurrentPackagePath, GetCurrentProcessorNumberEx,
    GetCurrentThreadStackLimits, GetFeatureToggleConfiguration, GetFeatureTogglesChangeToken,
    GetIoRingInfo, GetLogicalProcessorInformationEx, GetMachineTypeAttributes,
    GetNumaNodeProcessorMask2, GetPackageApplicationIds, GetPackageFamilyName, GetPackageFullName,
    GetPackageId, GetPackageInfo, GetPackagePath, GetPackagePathByFullName,
    GetPackagesByPackageFamily, GetProcessDefaultCpuSetMasks, GetProcessDefaultCpuSets,
    GetProcessMitigationPolicy, GetProcessorSystemCycleTime, GetStagedPackagePathByFullName,
    GetStateFolder, GetSystemAppDataKey, GetSystemCpuSetInformation, GetSystemFirmwareTable,
    GetThreadDescription, GetThreadSelectedCpuSetMasks, GetThreadSelectedCpuSets,
    GetTimeZoneInformationForYear, GetTraceEnableFlags, GetTraceEnableLevel, GetTraceLoggerHandle,
    GetUserDefaultLangID, GetUserDefaultUILanguage, GetVersion, HeapSetInformation,
    InitOnceComplete, InitializeCriticalSectionAndSpinCount, InitializeEnclave,
    InitializeProcThreadAttributeList, InitializeSListHead, InstallELAMCertificateInfo,
    InterlockedCompareExchange, InterlockedCompareExchange64, InterlockedExchange,
    InterlockedExchangeAdd, InterlockedPushEntrySList, InterlockedPushListSListEx,
    IsIoRingOpSupported, IsProcessCritical, IsWow64GuestMachineSupported, IsWow64Process2,
    LeaveCriticalSectionWhenCallbackReturns, LoadEnclaveData, LocalFileTimeToLocalSystemTime,
    LocalSystemTimeToLocalFileTime, OfferVirtualMemory, OutputDebugStringA, OutputDebugStringW,
    PackageFamilyNameFromFullName, PackageFamilyNameFromId, PackageFullNameFromId,
    PackageIdFromFullName, PackageNameAndPublisherIdFromFamilyName, ParseApplicationUserModelId,
    PopIoRingCompletion, PrefetchVirtualMemory, QueryDepthSList, QueryIoRingCapabilities,
    QueryProtectedPolicy, QueueUserAPC2, RaiseException, RaiseFailFastException,
    ReadConsoleInputExA, ReadConsoleInputExW, ReclaimVirtualMemory, RegisterTraceGuidsW,
    ResolveDelayLoadedAPI, ResolveDelayLoadsFromDll, RestoreLastError, SetConsoleInputExeNameA,
    SetConsoleInputExeNameW, SetCriticalSectionSpinCount, SetDefaultDllDirectories,
    SetEventWhenCallbackReturns, SetHandleCount, SetIoRingCompletionEvent, SetLocaleInfoW,
    SetProcessDefaultCpuSetMasks, SetProcessDefaultCpuSets, SetProcessDynamicEHContinuationTargets,
    SetProcessDynamicEnforcedCetCompatibleRanges, SetProcessMitigationPolicy,
    SetProcessWorkingSetSize, SetProtectedPolicy, SetThreadDescription, SetThreadLocale,
    SetThreadSelectedCpuSetMasks, SetThreadSelectedCpuSets, SetThreadStackGuarantee,
    SetThreadToken, SetThreadpoolThreadMaximum, SleepConditionVariableSRW, StartThreadpoolIo,
    SubmitIoRing, SubmitThreadpoolWork, SystemTimeToTzSpecificLocalTime,
    SystemTimeToTzSpecificLocalTimeEx, TerminateProcess, TlsGetValue2, TraceEvent, TraceMessage,
    TraceMessageVa, TzSpecificLocalTimeToSystemTimeEx, UpdateProcThreadAttribute, VirtualLock,
    Wow64Transition, WriteConsoleA, WriteConsoleW,
};
use crate::winapi::winapi64;

pub use crate::api::windows::common::process::GetCurrentProcessId;
pub use crate::api::windows::common::threads::{
    GetCurrentThreadId, GetThreadId, OpenThread, SuspendThread,
};
pub use crate::api::windows::libraries::kernel32::filesystem::{
    GetFileAttributesA, GetFileAttributesExA, GetFileAttributesExW, GetFileAttributesW,
};
pub use crate::api::windows::libraries::kernel32::library_loader::*;

pub use crate::api::windows::libraries::kernel32::system::{
    GetACP, GetOEMCP, GetThreadUILanguage, SetThreadUILanguage,
};
pub use crate::api::windows::libraries::kernel32::{
    CompareFileTime, DosDateTimeToFileTime, FileTimeToDosDateTime, FileTimeToLocalFileTime,
    FileTimeToSystemTime, GetTimeZoneInformation, LocalFileTimeToFileTime, SystemTimeToFileTime,
};

use crate::winapi::common::files::file_entry_points;

file_entry_points!(
    "kernelbase",
    CancelIo,
    CancelIoEx,
    CancelSynchronousIo,
    CopyFile2,
    CopyFileExW,
    CopyFileFromAppW,
    CopyFileW,
    CreateDirectory2A,
    CreateDirectory2W,
    CreateDirectoryA,
    CreateDirectoryExW,
    CreateDirectoryFromAppW,
    CreateDirectoryW,
    CreateFile2,
    CreateFile2FromAppW,
    CreateFile3,
    CreateFileFromAppW,
    CreateFileMapping2,
    CreateFileMappingFromApp,
    CreateFileMappingNumaW,
    CreateHardLinkA,
    CreateHardLinkW,
    CreateIoCompletionPort,
    CreateSymbolicLinkW,
    DefineDosDeviceW,
    DeleteFile2A,
    DeleteFile2W,
    DeleteFileA,
    DeleteFileFromAppW,
    DeleteFileW,
    DeleteVolumeMountPointW,
    FindClose,
    FindCloseChangeNotification,
    FindFirstChangeNotificationA,
    FindFirstChangeNotificationW,
    FindFirstFileA,
    FindFirstFileExA,
    FindFirstFileExFromAppW,
    FindFirstFileExW,
    FindFirstFileNameW,
    FindFirstFileW,
    FindFirstStreamW,
    FindFirstVolumeW,
    FindNextChangeNotification,
    FindNextFileA,
    FindNextFileNameW,
    FindNextFileW,
    FindNextStreamW,
    FindNextVolumeW,
    FindVolumeClose,
    FlushViewOfFile,
    GetCompressedFileSizeA,
    GetCompressedFileSizeW,
    GetCurrentDirectoryA,
    GetCurrentDirectoryW,
    GetDiskFreeSpaceExA,
    GetDiskFreeSpaceExW,
    GetDiskSpaceInformationA,
    GetDiskSpaceInformationW,
    GetDiskSpaceInformationWCOS,
    GetDriveTypeA,
    GetDriveTypeW,
    GetFileInformationByName,
    GetFileSecurityW,
    GetFileType,
    GetFinalPathNameByHandleA,
    GetFinalPathNameByHandleW,
    GetLogicalDriveStringsW,
    GetLogicalDrives,
    GetLongPathNameA,
    GetLongPathNameW,
    GetMappedFileNameA,
    GetMappedFileNameW,
    GetOverlappedResult,
    GetOverlappedResultEx,
    GetQueuedCompletionStatus,
    GetQueuedCompletionStatusEx,
    GetShortPathNameW,
    GetSystemDirectoryA,
    GetSystemDirectoryW,
    GetSystemFileCacheSize,
    GetSystemWindowsDirectoryA,
    GetSystemWindowsDirectoryW,
    GetSystemWow64Directory2A,
    GetSystemWow64Directory2W,
    GetSystemWow64DirectoryA,
    GetSystemWow64DirectoryW,
    GetTempFileNameA,
    GetTempFileNameW,
    GetTempPath2A,
    GetTempPath2W,
    GetTempPathA,
    GetTempPathW,
    GetVolumeInformationA,
    GetVolumeInformationByHandleW,
    GetVolumeInformationW,
    GetVolumeNameForVolumeMountPointW,
    GetVolumePathNameW,
    GetVolumePathNamesForVolumeNameW,
    GetWindowsDirectoryA,
    GetWindowsDirectoryW,
    K32GetMappedFileNameA,
    K32GetMappedFileNameW,
    LockFile,
    MapViewOfFile3,
    MapViewOfFile3FromApp,
    MapViewOfFileExNuma,
    MapViewOfFileFromApp,
    MapViewOfFileNuma2,
    MoveFileExW,
    MoveFileFromAppW,
    MoveFileWithProgressTransactedW,
    MoveFileWithProgressW,
    NeedCurrentDirectoryForExePathA,
    NeedCurrentDirectoryForExePathW,
    OpenFileById,
    PathFileExistsA,
    PathFileExistsW,
    PostQueuedCompletionStatus,
    QueryDosDeviceW,
    ReOpenFile,
    ReadDirectoryChangesExW,
    ReadDirectoryChangesW,
    ReadFileEx,
    ReadFileScatter,
    RemoveDirectory2A,
    RemoveDirectory2W,
    RemoveDirectoryA,
    RemoveDirectoryFromAppW,
    RemoveDirectoryW,
    ReplaceFileExInternal,
    ReplaceFileFromAppW,
    ReplaceFileW,
    SearchPathA,
    SearchPathW,
    SetFileAttributesA,
    SetFileAttributesFromAppW,
    SetFileAttributesW,
    SetFileInformationByHandle,
    SetFileIoOverlappedRange,
    SetFileSecurityW,
    SetFileValidData,
    SetSystemFileCacheSize,
    UnlockFile,
    UnlockFileEx,
    UnmapViewOfFile2,
    Wow64DisableWow64FsRedirection,
    Wow64EnableWow64FsRedirection,
    Wow64RevertWow64FsRedirection,
    WriteFileEx,
    WriteFileGather,
);

pub use crate::winapi::common::ntdll::{
    DecodePointer, EncodePointer, GetVersionExA, GetVersionExW, event_enabled as EventEnabled,
};

pub use crate::winapi::common::crt_arguments::{GetCommandLineA, GetCommandLineW};

mod app_policy_get_process_termination_method;
mod appcompat;
mod console;
mod diagnostics;
mod filesystem;
mod legacy_compat;
mod library_loader;
mod memory;
mod network;
mod objects;
mod process_thread;
mod registry_profile;
mod security;
mod strings_nls;
mod sync;
mod system;
mod time_date;
pub use app_policy_get_process_termination_method::AppPolicyGetProcessTerminationMethod;

mod event_unregister;
pub use event_unregister::EventUnregister;

mod open_file_mapping_from_app;
pub use open_file_mapping_from_app::OpenFileMappingFromApp;

mod virtual_alloc_from_app;
pub use virtual_alloc_from_app::VirtualAllocFromApp;
mod virtual_protect_from_app;
pub use virtual_protect_from_app::VirtualProtectFromApp;
mod __argc;
pub use __argc::__argc;

pub use crate::winapi::common::threads::{
    AddVectoredExceptionHandler, RemoveVectoredExceptionHandler, TlsAlloc, TlsFree, TlsGetValue,
    TlsSetValue,
};

mod create_file_mapping_w;
pub use create_file_mapping_w::CreateFileMappingW;
mod create_thread;
pub use create_thread::CreateThread;

mod event_register;
pub use event_register::EventRegister;
mod event_set_information;
pub use event_set_information::EventSetInformation;
mod event_write_transfer;
pub use event_write_transfer::EventWriteTransfer;
mod exit_process;
pub use exit_process::ExitProcess;
mod exit_thread;
pub use exit_thread::ExitThread;

mod flush_instruction_cache;
pub use flush_instruction_cache::FlushInstructionCache;

mod get_calendar_info_w;
pub use crate::winapi::common::handles::{GetConsoleMode, SetConsoleMode};
pub use get_calendar_info_w::GetCalendarInfoW;

pub use crate::winapi::libraries::kernel32::{
    GetLocalTime, GetSystemTime, GetSystemTimeAsFileTime, GetSystemTimePreciseAsFileTime,
};

pub use crate::winapi::common::heap::{GetProcessHeap, GetProcessHeaps};

mod get_thread_context;
pub use get_thread_context::GetThreadContext;

mod get_tick_count;
pub use get_tick_count::GetTickCount;
mod get_tick_count64;
pub use get_tick_count64::GetTickCount64;

mod global_alloc;
pub use global_alloc::GlobalAlloc;
mod global_free;
pub use crate::winapi::libraries::kernel32::memory::GlobalHandle;
pub use global_free::GlobalFree;
mod global_re_alloc;
pub use global_re_alloc::GlobalReAlloc;
mod global_size;
pub use global_size::GlobalSize;
mod heap_alloc;
pub use heap_alloc::HeapAlloc;
mod heap_create;
pub use heap_create::HeapCreate;
mod heap_destroy;
pub use heap_destroy::HeapDestroy;
mod heap_free;
pub use heap_free::HeapFree;
mod heap_re_alloc;
pub use heap_re_alloc::HeapReAlloc;

mod heap_size;
pub use heap_size::HeapSize;

mod local_free;
pub use local_free::LocalFree;
mod local_re_alloc;
pub use local_re_alloc::LocalReAlloc;
mod local_size;
pub use local_size::LocalSize;

mod map_view_of_file;
pub use map_view_of_file::MapViewOfFile;
mod map_view_of_file_ex;
pub use crate::winapi::libraries::kernel32::misc::MulDiv;
pub use map_view_of_file_ex::MapViewOfFileEx;

mod open_file_mapping_w;
pub use open_file_mapping_w::OpenFileMappingW;

mod query_performance_counter;
pub use query_performance_counter::QueryPerformanceCounter;
mod query_performance_frequency;
pub use query_performance_frequency::QueryPerformanceFrequency;

mod read_process_memory;
pub use read_process_memory::ReadProcessMemory;

pub use crate::winapi::registry::RegOpenCurrentUser;

mod resume_thread;
pub use resume_thread::ResumeThread;

mod set_thread_context;
pub use set_thread_context::SetThreadContext;

mod unmap_view_of_file_ex;
pub use unmap_view_of_file_ex::UnmapViewOfFileEx;

mod ver_set_condition_mask;
pub use ver_set_condition_mask::VerSetConditionMask;
mod virtual_alloc;
pub use virtual_alloc::VirtualAlloc;
mod virtual_alloc_ex;
pub use virtual_alloc_ex::VirtualAllocEx;
mod virtual_alloc_ex_numa;
pub use virtual_alloc_ex_numa::VirtualAllocExNuma;
mod virtual_free;
pub use virtual_free::VirtualFree;
mod virtual_free_ex;
pub use virtual_free_ex::VirtualFreeEx;

mod virtual_protect;
pub use virtual_protect::VirtualProtect;
mod virtual_protect_ex;
pub use virtual_protect_ex::VirtualProtectEx;
mod virtual_query;
pub use virtual_query::VirtualQuery;
mod virtual_query_ex;
pub use virtual_query_ex::VirtualQueryEx;

mod wide_char_to_multi_byte;
pub use wide_char_to_multi_byte::WideCharToMultiByte;

mod write_process_memory;
pub use write_process_memory::WriteProcessMemory;
mod lstrcmp;
pub use lstrcmp::{lstrcmp, lstrcmpA, lstrcmpW};

mod lstrcmpi;
pub use lstrcmpi::{lstrcmpi, lstrcmpiA, lstrcmpiW};

mod lstrcpyn;
pub use lstrcpyn::{lstrcpyn, lstrcpynA, lstrcpynW};

mod lstrlen;
pub use lstrlen::{lstrlen, lstrlenA, lstrlenW};

pub use crate::winapi::common::security::{GetTokenInformation, OpenProcessToken};
mod local_alloc;
use local_alloc::LocalAlloc;
mod _initterm;
use _initterm::_initterm;
mod _initterm_e;
use _initterm_e::_initterm_e;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.rsplit('!').next().unwrap_or(&api);
    gateway_by_name(addr, api, emu)
}

pub(super) fn gateway_by_name(addr: u64, api: &str, emu: &mut emu::Emu) -> String {
    if dispatch_shared(api, "kernelbase", emu) {
        return String::new();
    }
    if appcompat::dispatch(api, addr, emu)
        || console::dispatch(api, addr, emu)
        || diagnostics::dispatch(api, addr, emu)
        || filesystem::dispatch(api, addr, emu)
        || legacy_compat::dispatch(api, addr, emu)
        || library_loader::dispatch(api, addr, emu)
        || memory::dispatch(api, addr, emu)
        || network::dispatch(api, addr, emu)
        || objects::dispatch(api, addr, emu)
        || process_thread::dispatch(api, addr, emu)
        || registry_profile::dispatch(api, addr, emu)
        || security::dispatch(api, addr, emu)
        || strings_nls::dispatch(api, addr, emu)
        || sync::dispatch(api, addr, emu)
        || system::dispatch(api, addr, emu)
        || time_date::dispatch(api, addr, emu)
    {
        return String::new();
    }

    winapi64::kernel32::gateway_by_name(addr, api, emu)
}

unsupported_exports!(
    "kernelbase",
    DeleteTimerQueue,
    EnumSystemGeoID,
    EnumSystemGeoNames,
    GetGeoInfoEx,
    GetUserDefaultGeoName,
    GetUserGeoID,
    IsThreadAFiber,
    IsValidLocale,
    LocalFlags,
    RegisterApplicationRestart,
    SetSystemTimeAdjustment,
    SetUserGeoID,
    SetUserGeoName,
    UnregisterApplicationRestart,
    WerSetFlags,
    Wow64SuspendThread
);

unsupported_exports!(
    "kernelbase",
    CloseIoRing,
    ClosePackageInfo,
    CloseState,
    CloseThreadpool,
    CloseThreadpoolCleanupGroup,
    CloseThreadpoolCleanupGroupMembers,
    CloseThreadpoolIo,
    CloseThreadpoolWork,
    DeleteProcThreadAttributeList,
    FreeLibraryWhenCallbackReturns,
    RemoveDllDirectory,
    RemoveVectoredContinueHandler,
    UnregisterTraceGuids,
);
