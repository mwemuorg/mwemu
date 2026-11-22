use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::arch::x86::XCR0_ENABLED_MASK;
use crate::emu::Emu;
use crate::winapi::common::{
    critical_section, crt, crt_arguments, files, handles, heap, ipc, memory as virtual_memory,
    memory_ops, modules, ntdll, paths, process, resources, sections, security, shell_strings,
    strings, threads, wait,
};
use crate::winapi::libraries::{advapi32, version};
use crate::winapi::{policy, registry};

pub(crate) mod atoms;
pub(crate) mod entry_points;
pub mod environment;
pub mod filesystem;
pub mod library_loader;
mod locale;
pub mod memory;
mod message;
pub mod misc;
pub mod process_thread;
mod profile;
pub mod strings_nls;
mod sync;
pub mod system;

pub use crate::winapi::common::ntdll::event_enabled as EtwEventEnabled;

mod ver_set_condition_mask;
pub use ver_set_condition_mask::VerSetConditionMask;

mod time_zone;
pub use time_zone::{FileTimeToLocalFileTime, GetTimeZoneInformation, LocalFileTimeToFileTime};
pub(crate) mod file_time;
pub use file_time::{
    CompareFileTime, DosDateTimeToFileTime, FileTimeToDosDateTime, FileTimeToSystemTime,
    GetLocalTime, GetSystemTime, GetSystemTimeAsFileTime, GetSystemTimePreciseAsFileTime,
    SystemTimeToFileTime,
};

pub(crate) fn dispatch_shared(api: &str, library: &str, emu: &mut Emu) -> bool {
    if registry::kernel_gateway(api, emu)
        || environment::dispatch(api, emu)
        || sections::dispatch(api, emu)
        || memory_ops::dispatch(api, emu)
        || heap::dispatch(api, emu)
        || virtual_memory::dispatch(api, emu)
        || modules::dispatch(api, library, emu)
        || (library == "kernelbase" && version::dispatch(api, emu))
        || resources::dispatch(api, library, emu)
        || (library == "kernelbase" && paths::dispatch(api, library, emu))
        || wait::dispatch(api, library, emu)
        || ipc::dispatch(api, library, emu)
        || process::dispatch(api, library, emu)
        || files::dispatch(api, library, emu)
        || strings::dispatch(api, library, emu)
        || handles::dispatch(api, library, emu)
        || threads::dispatch(api, library, emu)
    {
        return true;
    }
    match api {
        "StringCbCopyW" | "StringCchCatW" | "StringCchCopyNW" | "StringCchCopyW"
        | "StringCchLengthW" => strings_nls::strsafe::strsafe(emu, library, api),
        "SetHandleCount" => handles::set_handle_count(emu, library),
        "DeviceIoControl" => files::device_io_control(emu, library),
        "GetVersion" => ntdll::packed_version(emu, library),
        "UnhandledExceptionFilter" => policy::unsupported(emu, library, api, emu.pc()),
        "GetCPInfo" | "IsValidCodePage" => locale::code_page_info(emu, library, api),
        "GetThreadLocale" | "SetThreadLocale" => locale::thread_locale(emu, library, api),
        "GetLastError" | "SetLastError" | "RestoreLastError" => {
            process::last_error(emu, library, api)
        }
        "GetProcessWorkingSetSize" => process::working_set(emu, library),
        "GetExitCodeProcessImplementation" => process::GetExitCodeProcess(emu),
        "SystemTimeToTzSpecificLocalTime" => time_zone::local_system_time(emu, library),
        "RaiseException" => process::raise_exception(emu, library),
        "OutputDebugStringA" | "OutputDebugStringW" => process::debug_output(emu, library, api),
        "FreeLibrary" => modules::free(emu, library),
        "DuplicateHandle" => handles::duplicate(emu, library, WinApi(Stdcall)),
        "SetLocaleInfoA" | "SetLocaleInfoW" | "StringCbCatExW" | "StringCbCopyExW"
        | "StringCchCopyExW" | "VerifyVersionInfoA" | "VerifyVersionInfoW" => {
            policy::unsupported(emu, library, api, emu.pc())
        }
        "GetPrivateProfileIntA"
        | "GetPrivateProfileIntW"
        | "GetPrivateProfileStringA"
        | "GetPrivateProfileStringW"
        | "GetProfileIntA"
        | "GetProfileIntW"
        | "GetProfileStringA"
        | "GetProfileStringW"
        | "WritePrivateProfileStringA"
        | "WritePrivateProfileStringW"
        | "WriteProfileStringA"
        | "WriteProfileStringW" => profile::profile(emu, library, api),
        "FormatMessageA" | "FormatMessageW" => message::format_message(emu, library, api),
        "InterlockedCompareExchange" | "InterlockedExchange" | "InterlockedExchangeAdd" => {
            sync::interlocked(emu, library, api)
        }
        "GetActiveProcessorCount" | "GetCurrentProcessorNumber" | "GetMaximumProcessorCount" => {
            process::processor_count(emu, library, api)
        }
        "IsDebuggerPresent" => process::debugger_present(emu, library),
        "FlushProcessWriteBuffers" => {
            log_red!(emu, "{library}!FlushProcessWriteBuffers");
        }
        "_initterm" | "_initterm_e" => {
            crt::dispatch_runtime(api, emu, crt::Runtime::Msvcrt);
        }
        "__chkstk" => sync::stack_probe(emu, library),
        "DeleteCriticalSection"
        | "EnterCriticalSection"
        | "InitializeCriticalSection"
        | "InitializeCriticalSectionAndSpinCount"
        | "InitializeCriticalSectionEx"
        | "LeaveCriticalSection"
        | "SetCriticalSectionSpinCount"
        | "TryEnterCriticalSection" => critical_section::section(emu, library, api),
        "AcquireSRWLockExclusive"
        | "AcquireSRWLockShared"
        | "InitOnceInitialize"
        | "InitializeConditionVariable"
        | "InitializeSRWLock"
        | "ReleaseSRWLockExclusive"
        | "ReleaseSRWLockShared"
        | "TryAcquireSRWLockExclusive"
        | "TryAcquireSRWLockShared"
        | "WakeAllConditionVariable"
        | "WakeConditionVariable" => sync::pointer_lock(emu, library, api),
        "InterlockedCompareExchange64" | "InterlockedDecrement" | "InterlockedIncrement" => {
            sync::interlocked(emu, library, api)
        }
        "Sleep" | "SleepEx" => sync::sleep(emu, library, api),
        "GetProcessAffinityMask" | "IsProcessCritical" => process::process_masks(emu, library, api),
        "GetStartupInfoA" | "GetStartupInfoW" => process::startup_info(emu, library, api),
        "CreateRemoteThread" | "CreateRemoteThreadEx" => threads::create_remote(emu, library, api),
        "LoadLibraryA" | "LoadLibraryExA" | "LoadLibraryExW" | "LoadLibraryW" => {
            modules::load(emu, library, api)
        }
        "GlobalFlags" | "GlobalLock" | "GlobalUnlock" => heap::local_lock(emu, api),
        "IsProcessorFeaturePresent" => process::processor_feature(emu, library),
        "BeginUpdateResourceA"
        | "BeginUpdateResourceW"
        | "CloseThreadpoolTimer"
        | "CloseThreadpoolWait"
        | "CreateEnclave"
        | "CreateIoRing"
        | "CreateProcessA"
        | "CreateProcessW"
        | "EndUpdateResourceA"
        | "EndUpdateResourceW"
        | "FindActCtxSectionStringA"
        | "FindActCtxSectionStringW"
        | "GetConsoleInputExeNameA"
        | "GetConsoleInputExeNameW"
        | "InterlockedFlushSList"
        | "InterlockedPopEntrySList"
        | "InterlockedPushEntrySList"
        | "InterlockedPushListSList"
        | "InterlockedPushListSListEx"
        | "IsThreadpoolTimerSet"
        | "LeaveCriticalSectionWhenCallbackReturns"
        | "NotifyFeatureToggleUsage"
        | "OpenPackageInfoByFullName"
        | "OpenState"
        | "OpenStateExplicit"
        | "ReadConsoleInputExA"
        | "ReadConsoleInputExW"
        | "ReleaseMutexWhenCallbackReturns"
        | "ReleaseSemaphoreWhenCallbackReturns"
        | "SetConsoleInputExeNameA"
        | "SetConsoleInputExeNameW"
        | "SetEventWhenCallbackReturns"
        | "SetIoRingCompletionEvent"
        | "SetLastConsoleEventActive"
        | "SleepConditionVariableCS"
        | "SleepConditionVariableSRW"
        | "UpdateResourceA"
        | "UpdateResourceW"
        | "WaitForDebugEventEx"
        | "WaitForThreadpoolIoCallbacks"
        | "WaitForThreadpoolTimerCallbacks"
        | "WaitForThreadpoolWaitCallbacks"
        | "WaitForThreadpoolWorkCallbacks" => policy::unsupported(emu, library, api, emu.pc()),
        "__C_specific_handler" | "__misaligned_access" | "__wargv" | "_local_unwind" => {
            policy::unsupported_with_abi(emu, library, api, emu.pc(), WinApi(Cdecl))
        }
        "OpenThreadToken" => threads::OpenThreadToken(emu),
        "IsEnclaveTypeSupported" | "IsUserCetAvailableInEnvironment" => {
            process::hardware_support(emu, library, api)
        }
        "CryptCreateHash" if library == "kernel32" => advapi32::CryptCreateHash(emu),
        "MultiByteToWideChar" => locale::multi_byte_to_wide(emu, library),
        "GetConsoleCP" | "GetConsoleOutputCP" => handles::console_code_page(emu, library, api),
        "HeapSetInformation" => policy::unsupported(emu, library, api, emu.pc()),
        "TerminateProcess" => process::terminate(emu, library),
        "GetNativeSystemInfo" | "GetSystemInfo" => process::system_info(emu, library, api),
        "IsWow64Process" | "IsWow64Process2" | "CheckRemoteDebuggerPresent" => {
            process::process_platform(emu, library, api)
        }
        "GetStringTypeA" | "GetStringTypeW" | "GetStringTypeExA" | "GetStringTypeExW" => {
            locale::character_types(emu, library, api)
        }
        "CompareStringA" | "CompareStringW" | "CompareStringOrdinal" => {
            locale::compare_text(emu, library, api)
        }
        "GetCommandLineA" => crt_arguments::GetCommandLineA(emu),
        "GetCommandLineW" => crt_arguments::GetCommandLineW(emu),
        "RtlCaptureContext" if library == "kernel32" => threads::capture_context(emu),
        "GetDateFormatA" | "GetDateFormatW" | "GetDateFormatEx" | "GetTimeFormatA"
        | "GetTimeFormatW" | "GetTimeFormatEx" => file_time::format_local(emu, library, api),
        "WaitOnAddress" | "WakeByAddressAll" | "WakeByAddressSingle" => {
            wait::address_wait(emu, library, api)
        }
        "FindStringOrdinal" => shell_strings::find_ordinal(emu),
        "LCMapStringEx" | "LCMapStringA" | "LCMapStringW" => {
            shell_strings::map_locale_case(emu, library, api)
        }
        "GetLocaleInfoEx" | "GetLocaleInfoA" | "GetLocaleInfoW" => {
            locale::locale_info(emu, library, api)
        }
        "GetCurrentPackagePath" => process::current_package_path(emu, library),
        "FlsAlloc" | "FlsFree" | "FlsGetValue" | "FlsGetValue2" | "FlsSetValue" => {
            threads::fls(emu, library, api)
        }
        "WriteConsoleA" | "WriteConsoleW" => handles::write_console(emu, library, api),
        "GetTimeZoneInformationForYear" => time_zone::year(emu, library),
        "GetSystemFirmwareTable"
        | "SetThreadStackGuarantee"
        | "ActivateActCtx"
        | "InitOnceBeginInitialize"
        | "InitOnceExecuteOnce"
        | "SetThreadpoolTimer"
        | "SetThreadpoolTimerEx"
        | "SetThreadpoolWait"
        | "SetThreadpoolWaitEx"
        | "CreateActCtxA"
        | "CreateActCtxW" => policy::unsupported(emu, library, api, emu.pc()),
        "HeapQueryInformation" => heap::query_compatibility(emu, library),
        "GetThreadUILanguage" => system::GetThreadUILanguage(emu),
        "SetThreadUILanguage" => system::SetThreadUILanguage(emu),
        "RtlLookupFunctionEntry" if !emu.cfg.arch.is_64bits() => {
            policy::unsupported(emu, library, api, emu.pc())
        }
        "RtlVirtualUnwind" if !emu.cfg.arch.is_64bits() => {
            policy::unsupported(emu, library, api, emu.pc())
        }
        "TraceMessage" | "__wgetmainargs" | "_amsg_exit" | "_c_exit" | "_cexit"
        | "_invalid_parameter" | "_onexit" | "_purecall" | "_time64" | "atexit" | "time"
        | "wprintf"
            if library == "kernelbase" =>
        {
            policy::unsupported_with_abi(emu, library, api, emu.pc(), WinApi(Cdecl))
        }
        "AddAtomA" | "AddAtomW" | "FindAtomA" | "FindAtomW" | "GetAtomNameA" | "GetAtomNameW"
        | "GlobalAddAtomA" | "GlobalAddAtomW" | "GlobalFindAtomA" | "GlobalFindAtomW"
        | "GlobalGetAtomNameA" | "GlobalGetAtomNameW" | "DeleteAtom" | "GlobalDeleteAtom"
        | "GlobalAddAtomExA" | "GlobalAddAtomExW" => atoms::atom(api, library, emu),
        "QueryInterruptTime"
        | "QueryInterruptTimePrecise"
        | "QueryUnbiasedInterruptTime"
        | "QueryUnbiasedInterruptTimePrecise" => file_time::interrupt_time(emu, api),
        "SetConsoleCursorInfo" | "GetConsoleCursorInfo" => {
            handles::cursor_info(emu, api == "SetConsoleCursorInfo")
        }
        "QueryFullProcessImageNameA" | "QueryFullProcessImageNameW" => {
            process::process_image(emu, api)
        }
        "StrRChrA" | "StrRChrW" | "StrRChrIA" | "StrRChrIW" | "StrRStrIA" | "StrRStrIW"
            if library == "kernelbase" =>
        {
            shell_strings::reverse_search(emu, api)
        }
        "StrCmpNA" | "StrCmpNW" | "StrCmpNIA" | "StrCmpNIW" if library == "kernelbase" => {
            shell_strings::compare_bounded(emu, library, api)
        }
        "StrChrA" | "StrChrW" | "StrChrIA" | "StrChrIW" | "StrStrA" | "StrStrW" | "StrStrIA"
        | "StrStrIW" | "StrPBrkA" | "StrPBrkW" | "StrSpnA" | "StrSpnW" | "StrCSpnA"
        | "StrCSpnW" | "StrCSpnIA" | "StrCSpnIW"
            if library == "kernelbase" =>
        {
            shell_strings::search(emu, api)
        }
        "StrTrimA" | "StrTrimW" if library == "kernelbase" => shell_strings::trim(emu, api),
        "CharLowerA" | "CharLowerW" | "CharLowerBuffA" | "CharLowerBuffW" | "CharUpperA"
        | "CharUpperW" | "CharUpperBuffA" | "CharUpperBuffW"
            if library == "kernelbase" =>
        {
            shell_strings::char_case(emu, api)
        }
        "GetSystemDefaultLCID"
        | "GetUserDefaultLCID"
        | "GetSystemDefaultUILanguage"
        | "GetUserDefaultLangID"
        | "GetUserDefaultUILanguage"
        | "GetSystemDefaultLangID"
        | "GetUserDefaultLocaleName"
        | "GetSystemDefaultLocaleName"
        | "LCIDToLocaleName"
        | "LocaleNameToLCID"
        | "IsValidLocaleName" => locale::locale(emu, library, api),
        "IsBadStringPtrA" | "IsBadStringPtrW" => strings::bad_string_pointer(emu, api),
        "LocalLock" | "LocalUnlock" => heap::local_lock(emu, api),
        "GetNumberOfConsoleInputEvents" => handles::input_event_count(emu),
        "GetEnabledXStateFeatures" | "GetThreadEnabledXStateFeatures" => enabled_xstate(emu, api),
        "GetSystemPreferredUILanguages"
        | "GetThreadPreferredUILanguages"
        | "GetUserPreferredUILanguages" => locale::preferred_languages(emu, library, api),
        "GetPriorityClass" | "GetThreadPriority" | "SetPriorityClass" | "SetThreadPriority" => {
            process::priority(emu, api)
        }
        "GetComputerNameExA" | "GetComputerNameExW" => strings::computer_name_ex(emu, api),
        "GlobalMemoryStatusEx" | "GetPhysicallyInstalledSystemMemory" => {
            virtual_memory::memory_status(emu, api)
        }
        "lstrcat" => strings::concatenate(emu, api, false),
        "lstrcmp" => strings::compare(emu, api, false, false),
        "exit" | "_exit" if library == "kernelbase" => threads::terminate_crt(emu, api),
        _ if library == "kernelbase" && security::dispatch_sid(api, library, emu) => {}
        _ => return false,
    }
    true
}

fn enabled_xstate(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    // The CPU model supplies XCR0 through XGETBV.
    log_red!(emu, "{api}");
    call.set_return_u64(emu, XCR0_ENABLED_MASK);
}
