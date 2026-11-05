use crate::winapi::helper;
use crate::{emu, serialization};

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

pub use appcompat::*;
pub use console::*;
pub use filesystem::*;
pub use legacy_compat::*;
pub use library_loader::*;
pub use loader::*;
pub use memory::*;
pub use misc::*;
pub use process_thread::*;
pub use registry_profile::*;
pub use resolver::*;
pub use state::*;
pub use strings_nls::*;
pub use sync::*;
pub use system::*;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "AddVectoredExceptionHandler" => AddVectoredExceptionHandler(emu),
        "GlobalAlloc" => GlobalAlloc(emu),
        "GlobalFree" => GlobalFree(emu),
        "LocalFree" => LocalFree(emu),
        "GetTickCount64" => GetTickCount64(emu),
        "QueryPerformanceFrequency" => QueryPerformanceFrequency(emu),
        "IsWow64Process" => IsWow64Process(emu),
        "OutputDebugStringA" => OutputDebugStringA(emu),
        "OutputDebugStringW" => OutputDebugStringW(emu),
        "FormatMessageA" => FormatMessageA(emu),
        "FormatMessageW" => FormatMessageW(emu),
        "DuplicateHandle" => DuplicateHandle(emu),
        "InitializeSRWLock" => InitializeSRWLock(emu),
        "AcquireSRWLockExclusive" => AcquireSRWLockExclusive(emu),
        "AcquireSRWLockShared" => AcquireSRWLockShared(emu),
        "ReleaseSRWLockExclusive" => ReleaseSRWLockExclusive(emu),
        "ReleaseSRWLockShared" => ReleaseSRWLockShared(emu),
        "TryAcquireSRWLockExclusive" => TryAcquireSRWLockExclusive(emu),
        "TryAcquireSRWLockShared" => TryAcquireSRWLockShared(emu),
        "ResetEvent" => ResetEvent(emu),
        "WaitForMultipleObjects" => WaitForMultipleObjects(emu),
        "ExitThread" => ExitThread(emu),
        "CreateProcessW" => CreateProcessW(emu),
        "Process32First" => Process32First(emu),
        "CreateDirectoryA" => CreateDirectoryA(emu),
        "CreateDirectoryW" => CreateDirectoryW(emu),
        "DeleteFileW" => DeleteFileW(emu),
        "CreateFileA" => CreateFileA(emu),
        "SetCurrentDirectoryA" => SetCurrentDirectoryA(emu),
        "lstrlenW" => lstrlenW(emu),
        "lstrcpyW" => lstrcpyW(emu),
        "lstrcpyn" => lstrcpyn(emu),
        "lstrcatW" => lstrcatW(emu),
        "lstrcmpiW" => lstrcmpiW(emu),
        "CompareStringW" => CompareStringW(emu),
        "GetLocaleInfoA" => GetLocaleInfoA(emu),
        "GetLocaleInfoW" => GetLocaleInfoW(emu),
        "GetUserDefaultLCID" => GetUserDefaultLCID(emu),
        "GetThreadLocale" => GetThreadLocale(emu),
        "GetVersionExA" => GetVersionExA(emu),
        "SystemTimeToFileTime" => SystemTimeToFileTime(emu),
        "SetEnvironmentVariableA" => SetEnvironmentVariableA(emu),
        "SetEnvironmentVariableW" => SetEnvironmentVariableW(emu),
        "GetEnvironmentVariableW" => GetEnvironmentVariableW(emu),
        "GetModuleHandleExW" => GetModuleHandleExW(emu),
        "GetConsoleMode" => GetConsoleMode(emu),
        "GetConsoleCP" => GetConsoleCP(emu),
        "GetConsoleOutputCP" => GetConsoleOutputCP(emu),
        "WriteConsoleW" => WriteConsoleW(emu),
        "lstrlenA" => lstrlen(emu),
        "lstrcatA" => lstrcat(emu),
        "GetFileSize" => GetFileSize(emu),
        "GetFileSizeEx" => GetFileSizeEx(emu),
        "AreFileApiIsAnsi" | "AreFileApisANSI" => AreFileApisANSI(emu),
        "CloseHandle" => CloseHandle(emu),
        "ConnectNamedPipe" => ConnectNamedPipe(emu),
        "DeleteFileA" => DeleteFileA(emu),
        "CopyFileA" => CopyFileA(emu),
        "CopyFileW" => CopyFileW(emu),
        "CreateEventA" => CreateEventA(emu),
        "CreateFileMappingA" => CreateFileMappingA(emu),
        "CreateFileMappingW" => CreateFileMappingW(emu),
        "CreateFileW" => CreateFileW(emu),
        "CreateMutexA" => CreateMutexA(emu),
        "CreateMutexW" => CreateMutexW(emu),
        "CreateNamedPipeA" => CreateNamedPipeA(emu),
        "CreateProcessA" => CreateProcessA(emu),
        "CreateRemoteThread" => CreateRemoteThread(emu),
        "CreateThread" => CreateThread(emu),
        "CreateToolhelp32Snapshot" => CreateToolhelp32Snapshot(emu),
        "CryptCreateHash" => CryptCreateHash(emu),
        "DecodePointer" => DecodePointer(emu),
        "DisconnectNamedPipe" => DisconnectNamedPipe(emu),
        "EncodePointer" => EncodePointer(emu),
        "EnterCriticalSection" => EnterCriticalSection(emu),
        "ExitProcess" => ExitProcess(emu),
        "ExpandEnvironmentStringsA" => ExpandEnvironmentStringsA(emu),
        "ExpandEnvironmentStringsW" => ExpandEnvironmentStringsW(emu),
        "FileTimeToDosDateTime" => FileTimeToDosDateTime(emu),
        "FileTimeToLocalFileTime" => FileTimeToLocalFileTime(emu),
        "FileTimeToSystemTime" => FileTimeToSystemTime(emu),
        "FindClose" => FindClose(emu),
        "FindFirstFileA" => FindFirstFileA(emu),
        "FindFirstFileW" => FindFirstFileW(emu),
        "FindNextFileA" => FindNextFileA(emu),
        "FindNextFileW" => FindNextFileW(emu),
        "FindResourceA" => FindResourceA(emu),
        "FindResourceW" => FindResourceW(emu),
        "FlsAlloc" => FlsAlloc(emu),
        "FlsGetValue" => FlsGetValue(emu),
        "FlsSetValue" => FlsSetValue(emu),
        "FreeLibrary" => FreeLibrary(emu),
        "FreeResource" => FreeResource(emu),
        "GetACP" => GetACP(emu),
        "GetThreadId" => GetThreadId(emu),
        "GetCommandLineA" => GetCommandLineA(emu),
        "GetCommandLineW" => GetCommandLineW(emu),
        "GetComputerNameA" => GetComputerNameA(emu),
        "GetCPInfo" => GetCPInfo(emu),
        "GetCurrentDirectoryA" => GetCurrentDirectoryA(emu),
        "GetCurrentDirectoryW" => GetCurrentDirectoryW(emu),
        "GetCurrentProcess" => GetCurrentProcess(emu),
        "GetCurrentProcessId" => GetCurrentProcessId(emu),
        "GetDiskFreeSpaceA" => GetDiskFreeSpaceA(emu),
        "GetDiskFreeSpaceExA" => GetDiskFreeSpaceExA(emu),
        "GetDiskFreeSpaceExW" => GetDiskFreeSpaceExW(emu),
        "GetDriveTypeA" => GetDriveTypeA(emu),
        "GetDriveTypeW" => GetDriveTypeW(emu),
        "GetCurrentThreadId" => GetCurrentThreadId(emu),
        "FreeEnvironmentStringsA" => FreeEnvironmentStringsA(emu),
        "FreeEnvironmentStringsW" => FreeEnvironmentStringsW(emu),
        "GetEnvironmentStrings" => GetEnvironmentStrings(emu),
        "GetEnvironmentStringsW" => GetEnvironmentStringsW(emu),
        "GetFileAttributesA" => GetFileAttributesA(emu),
        "GetFileAttributesW" => GetFileAttributesW(emu),
        "GetFileType" => GetFileType(emu),
        "GetFullPathNameA" => GetFullPathNameA(emu),
        "GetFullPathNameW" => GetFullPathNameW(emu),
        "GetLastError" => GetLastError(emu),
        "GetLogicalDrives" => GetLogicalDrives(emu),
        "GetLocalTime" => GetLocalTime(emu),
        "GetLongPathNameW" => GetLongPathNameW(emu),
        "GetModuleFileNameA" => GetModuleFileNameA(emu),
        "GetModuleFileNameW" => GetModuleFileNameW(emu),
        "GetModuleHandleA" => GetModuleHandleA(emu),
        "GetModuleHandleW" => GetModuleHandleW(emu),
        "GetNativeSystemInfo" => GetNativeSystemInfo(emu),
        "GetOEMCP" => GetOEMCP(emu),
        "GetProcAddress" => GetProcAddress(emu),
        "GetProcessAffinityMask" => GetProcessAffinityMask(emu),
        "GetProcessHeap" => GetProcessHeap(emu),
        "GetStartupInfoA" => GetStartupInfoA(emu),
        "GetStartupInfoW" => GetStartupInfoW(emu),
        "GetStdHandle" => GetStdHandle(emu),
        "GetStringTypeW" => GetStringTypeW(emu),
        "GetSystemDirectoryA" => GetSystemDirectoryA(emu),
        "GetSystemDirectoryW" => GetSystemDirectoryW(emu),
        "GetSystemInfo" => GetSystemInfo(emu),
        "GetSystemTime" => GetSystemTime(emu),
        "GetSystemTimeAsFileTime" => GetSystemTimeAsFileTime(emu),
        "GetSystemWindowsDirectoryA" => GetSystemWindowsDirectoryA(emu),
        "GetSystemWindowsDirectoryW" => GetSystemWindowsDirectoryW(emu),
        "GetTempPathA" => GetTempPathA(emu),
        "GetTempPathW" => GetTempPathW(emu),
        "GetThreadContext" => GetThreadContext(emu),
        "GetThreadPreferredUILanguages" => GetThreadPreferredUILanguages(emu),
        "GetThreadUILanguage" => GetThreadUILanguage(emu),
        "GetTickCount" => GetTickCount(emu),
        "GetTimeZoneInformation" => GetTimeZoneInformation(emu),
        "GetUserDefaultLangID" => GetUserDefaultLangID(emu),
        "GetUserDefaultUILanguage" => GetUserDefaultUILanguage(emu),
        "GetVersion" => GetVersion(emu),
        "GetVersionExW" => GetVersionExW(emu),
        "GetVolumeInformationA" => GetVolumeInformationA(emu),
        "GetVolumeInformationW" => GetVolumeInformationW(emu),
        "GetWindowsDirectoryA" => GetWindowsDirectoryA(emu),
        "GetWindowsDirectoryW" => GetWindowsDirectoryW(emu),
        "GlobalAddAtomA" => GlobalAddAtomA(emu),
        "HeapAlloc" => HeapAlloc(emu),
        "HeapCreate" => HeapCreate(emu),
        "HeapDestroy" => HeapDestroy(emu),
        "HeapFree" => HeapFree(emu),
        "HeapReAlloc" => HeapReAlloc(emu),
        "HeapSetInformation" => HeapSetInformation(emu),
        "HeapSize" => HeapSize(emu),
        "InitializeCriticalSection" => InitializeCriticalSection(emu),
        "InitializeCriticalSectionAndSpinCount" => InitializeCriticalSectionAndSpinCount(emu),
        "InitializeCriticalSectionEx" => InitializeCriticalSectionEx(emu),
        "InterlockedIncrement" => InterlockedIncrement(emu),
        "IsDebuggerPresent" => IsDebuggerPresent(emu),
        "IsProcessorFeaturePresent" => IsProcessorFeaturePresent(emu),
        "IsValidCodePage" => IsValidCodePage(emu),
        "IsValidLocale" => IsValidLocale(emu),
        "LCMapStringW" => LCMapStringW(emu),
        "LeaveCriticalSection" => LeaveCriticalSection(emu),
        "LoadLibraryA" => LoadLibraryA(emu),
        "LoadLibraryExA" => LoadLibraryExA(emu),
        "LoadLibraryExW" => LoadLibraryExW(emu),
        "LoadLibraryW" => LoadLibraryW(emu),
        "LoadResource" => LoadResource(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "LockResource" => LockResource(emu),
        "lstrcat" => lstrcat(emu),
        "lstrcpy" => lstrcpy(emu),
        "lstrlen" => lstrlen(emu),
        "MapViewOfFile" => MapViewOfFile(emu),
        "Module32First" => Module32First(emu),
        "Module32FirstW" => Module32FirstW(emu),
        "Module32Next" => Module32Next(emu),
        "Module32NextW" => Module32NextW(emu),
        "MoveFileA" => MoveFileA(emu),
        "MoveFileW" => MoveFileW(emu),
        "MultiByteToWideChar" => MultiByteToWideChar(emu),
        "OpenProcess" => OpenProcess(emu),
        "OpenProcessToken" => OpenProcessToken(emu),
        "OpenThread" => OpenThread(emu),
        "QueryPerformanceCounter" => QueryPerformanceCounter(emu),
        "RaiseException" => RaiseException(emu),
        "ReadFile" => ReadFile(emu),
        "ReadProcessMemory" => ReadProcessMemory(emu),
        "RegCloseKey" => RegCloseKey(emu),
        "RegCreateKeyExA" => RegCreateKeyExA(emu),
        "RegCreateKeyExW" => RegCreateKeyExW(emu),
        "RegOpenKeyA" => RegOpenKeyA(emu),
        "RegOpenKeyExW" => RegOpenKeyExW(emu),
        "RegOpenKeyW" => RegOpenKeyW(emu),
        "RegSetValueExA" => RegSetValueExA(emu),
        "RegSetValueExW" => RegSetValueExW(emu),
        "RemoveDirectoryA" => RemoveDirectoryA(emu),
        "RemoveDirectoryW" => RemoveDirectoryW(emu),
        "ResumeThread" => ResumeThread(emu),
        "SetErrorMode" => SetErrorMode(emu),
        "SetFileAttributesA" => SetFileAttributesA(emu),
        "SetFileAttributesW" => SetFileAttributesW(emu),
        "SetFilePointer" => SetFilePointer(emu),
        "SetHandleCount" => SetHandleCount(emu),
        "SetLastError" => SetLastError(emu),
        "SetThreadContext" => SetThreadContext(emu),
        "SetThreadLocale" => SetThreadLocale(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "SizeofResource" => SizeofResource(emu),
        "Sleep" => Sleep(emu),
        "SystemTimeToTzSpecificLocalTime" => SystemTimeToTzSpecificLocalTime(emu),
        "TerminateProcess" => TerminateProcess(emu),
        "Thread32First" => Thread32First(emu),
        "Thread32Next" => Thread32Next(emu),
        "TlsAlloc" => TlsAlloc(emu),
        "TlsFree" => TlsFree(emu),
        "TlsGetValue" => TlsGetValue(emu),
        "TlsSetValue" => TlsSetValue(emu),
        "UnhandledExceptionFilter" => UnhandledExceptionFilter(emu),
        "VerifyVersionInfoW" => VerifyVersionInfoW(emu),
        "VirtualAlloc" => VirtualAlloc(emu),
        "VirtualAllocEx" => VirtualAllocEx(emu),
        "VirtualAllocExNuma" => VirtualAllocExNuma(emu),
        "VirtualLock" => VirtualLock(emu),
        "VirtualFree" => VirtualFree(emu),

        "VirtualProtect" => VirtualProtect(emu),
        "VirtualProtectEx" => VirtualProtectEx(emu),
        "VirtualQuery" => VirtualQuery(emu),
        "VirtualQueryEx" => VirtualQueryEx(emu),
        "WaitForSingleObject" => WaitForSingleObject(emu),
        "WideCharToMultiByte" => WideCharToMultiByte(emu),
        "WinExec" => WinExec(emu),
        "WriteFile" => WriteFile(emu),
        "WriteProcessMemory" => WriteProcessMemory(emu),
        "_TlgCreateWsz" => _TlgCreateWsz(emu),
        "_lclose" => _lclose(emu),
        "_lcreat" => _lcreat(emu),
        "_llseek" => _llseek(emu),
        "_lopen" => _lopen(emu),
        "_lread" => _lread(emu),
        "_lwrite" => _lwrite(emu),
        "ActivateActCtxWorker" => ActivateActCtxWorker(emu),
        "AddAtomA" => AddAtomA(emu),
        "AddAtomW" => AddAtomW(emu),
        "AddIntegrityLabelToBoundaryDescriptor" => AddIntegrityLabelToBoundaryDescriptor(emu),
        "AddLocalAlternateComputerNameW" => AddLocalAlternateComputerNameW(emu),
        "AdjustCalendarDate" => AdjustCalendarDate(emu),
        "ApplicationRecoveryFinished" => ApplicationRecoveryFinished(emu),
        "ApplicationRecoveryInProgress" => ApplicationRecoveryInProgress(emu),
        "BackupRead" => BackupRead(emu),
        "BackupSeek" => BackupSeek(emu),
        "BackupWrite" => BackupWrite(emu),
        "BaseCheckAppcompatCacheExWorker" => BaseCheckAppcompatCacheExWorker(emu),
        "BaseCheckAppcompatCacheWorker" => BaseCheckAppcompatCacheWorker(emu),
        "BaseCheckElevation" => BaseCheckElevation(emu),
        "BaseCleanupAppcompatCacheSupportWorker" => BaseCleanupAppcompatCacheSupportWorker(emu),
        "BaseDestroyVDMEnvironment" => BaseDestroyVDMEnvironment(emu),
        "BaseDllInitialize" => BaseDllInitialize(emu),
        "BaseDllReadWriteIniFile" => BaseDllReadWriteIniFile(emu),
        "BaseExitThreadPoolThread" => BaseExitThreadPoolThread(emu),
        "BaseFlushAppcompatCacheWorker" => BaseFlushAppcompatCacheWorker(emu),
        "BaseFreeAppCompatDataForProcessWorker" => BaseFreeAppCompatDataForProcessWorker(emu),
        "BaseGenerateAppCompatData" => BaseGenerateAppCompatData(emu),
        "BaseIsDosApplication" => BaseIsDosApplication(emu),
        "BaseReadAppCompatDataForProcessWorker" => BaseReadAppCompatDataForProcessWorker(emu),
        "BaseSetLastNTError" => BaseSetLastNTError(emu),
        "BaseUpdateAppcompatCacheWorker" => BaseUpdateAppcompatCacheWorker(emu),
        "BaseUpdateVDMEntry" => BaseUpdateVDMEntry(emu),
        "BaseVerifyUnicodeString" => BaseVerifyUnicodeString(emu),
        "BaseWriteErrorElevationRequiredEvent" => BaseWriteErrorElevationRequiredEvent(emu),
        "Basep8BitStringToDynamicUnicodeString" => Basep8BitStringToDynamicUnicodeString(emu),
        "BasepAccumulateIoRateControlInformationBufferSize" => {
            BasepAccumulateIoRateControlInformationBufferSize(emu)
        }
        "BasepAllocateActivationContextActivationBlock" => {
            BasepAllocateActivationContextActivationBlock(emu)
        }
        "BasepAnsiStringToDynamicUnicodeString" => BasepAnsiStringToDynamicUnicodeString(emu),
        "BasepAppContainerEnvironmentExtension" => BasepAppContainerEnvironmentExtension(emu),
        "BasepAppXExtension" => BasepAppXExtension(emu),
        "BasepCheckAppCompat" => BasepCheckAppCompat(emu),
        "BasepCheckWebBladeHashes" => BasepCheckWebBladeHashes(emu),
        "BasepCheckWinSaferRestrictions" => BasepCheckWinSaferRestrictions(emu),
        "BasepConfigureAppCertDlls" => BasepConfigureAppCertDlls(emu),
        "BasepConstructSxsCreateProcessMessage" => BasepConstructSxsCreateProcessMessage(emu),
        "BasepCopyEncryption" => BasepCopyEncryption(emu),
        "BasepFreeActivationContextActivationBlock" => {
            BasepFreeActivationContextActivationBlock(emu)
        }
        "BasepFreeAppCompatData" => BasepFreeAppCompatData(emu),
        "BasepGetComputerNameFromNtPath" => BasepGetComputerNameFromNtPath(emu),
        "BasepGetExeArchType" => BasepGetExeArchType(emu),
        "BasepGetMiniVersionForCreate" => BasepGetMiniVersionForCreate(emu),
        "BasepInitAppCompatData" => BasepInitAppCompatData(emu),
        "BasepInitializeApphelpGlobals" => BasepInitializeApphelpGlobals(emu),
        "BasepInitializeTermsrvFpns" => BasepInitializeTermsrvFpns(emu),
        "BasepIsProcessAllowed" => BasepIsProcessAllowed(emu),
        "BasepIsTestSigningEnabled" => BasepIsTestSigningEnabled(emu),
        "BasepNotifyLoadStringResource" => BasepNotifyLoadStringResource(emu),
        "BasepPostSuccessAppXExtension" => BasepPostSuccessAppXExtension(emu),
        "BasepProcessInvalidImage" => BasepProcessInvalidImage(emu),
        "BasepQueryAppCompat" => BasepQueryAppCompat(emu),
        "BasepRegenerateActCtxWithLanguage" => BasepRegenerateActCtxWithLanguage(emu),
        "BasepReleaseSxsCreateProcessUtilityStruct" => {
            BasepReleaseSxsCreateProcessUtilityStruct(emu)
        }
        "BasepReportFault" => BasepReportFault(emu),
        "BasepSetFileEncryptionCompression" => BasepSetFileEncryptionCompression(emu),
        "BasepSxsCreateStreams" => BasepSxsCreateStreams(emu),
        "BasepTpIoCallback" => BasepTpIoCallback(emu),
        "BasepTpIoCleanupCallback" => BasepTpIoCleanupCallback(emu),
        "BasepTpIoFinalizationCallback" => BasepTpIoFinalizationCallback(emu),
        "BeepImplementation" => BeepImplementation(emu),
        "BeginUpdateResourceA" => BeginUpdateResourceA(emu),
        "BeginUpdateResourceW" => BeginUpdateResourceW(emu),
        "BindIoCompletionCallback" => BindIoCompletionCallback(emu),
        "BuildCommDCBAndTimeoutsA" => BuildCommDCBAndTimeoutsA(emu),
        "BuildCommDCBAndTimeoutsW" => BuildCommDCBAndTimeoutsW(emu),
        "BuildCommDCBA" => BuildCommDCBA(emu),
        "BuildCommDCBW" => BuildCommDCBW(emu),
        "ByteMatch" => ByteMatch(emu),
        "CalibrateTimer" => CalibrateTimer(emu),
        "CallNamedPipeA" => CallNamedPipeA(emu),
        "CancelTimerQueueTimer" => CancelTimerQueueTimer(emu),
        "CheckElevation" => CheckElevation(emu),
        "CheckForReadOnlyResource" => CheckForReadOnlyResource(emu),
        "CheckNameLegalDOS8Dot3A" => CheckNameLegalDOS8Dot3A(emu),
        "CheckNameLegalDOS8Dot3W" => CheckNameLegalDOS8Dot3W(emu),
        "CloseConsoleHandle" => CloseConsoleHandle(emu),
        "CommConfigDialogA" => CommConfigDialogA(emu),
        "CompareCalendarDates" => CompareCalendarDates(emu),
        "CompatCacheLookupExe" => CompatCacheLookupExe(emu),
        "ConsoleMenuControl" => ConsoleMenuControl(emu),
        "ConvertCalDateTimeToSystemTime" => ConvertCalDateTimeToSystemTime(emu),
        "ConvertNLSDayOfWeekToWin32DayOfWeek" => ConvertNLSDayOfWeekToWin32DayOfWeek(emu),
        "ConvertSystemTimeToCalDateTime" => ConvertSystemTimeToCalDateTime(emu),
        "CopyFileExA" => CopyFileExA(emu),
        "CopyFileTransactedA" => CopyFileTransactedA(emu),
        "CopyFileTransactedW" => CopyFileTransactedW(emu),
        "CopyLZFile" => CopyLZFile(emu),
        "CptpQuirksInitOnce" => CptpQuirksInitOnce(emu),
        "CreateActCtxA" => CreateActCtxA(emu),
        "CreateActCtxWWorker" => CreateActCtxWWorker(emu),
        "CreateBoundaryDescriptorA" => CreateBoundaryDescriptorA(emu),
        "CreateDirectoryExA" => CreateDirectoryExA(emu),
        "CreateDirectoryTransactedA" => CreateDirectoryTransactedA(emu),
        "CreateDirectoryTransactedW" => CreateDirectoryTransactedW(emu),
        "CreateFileMappingNumaA" => CreateFileMappingNumaA(emu),
        "CreateHardLinkTransactedA" => CreateHardLinkTransactedA(emu),
        "CreateSemaphoreA" => CreateSemaphoreA(emu),
        "CreateSemaphoreExA" => CreateSemaphoreExA(emu),
        "CreateSocketHandle" => CreateSocketHandle(emu),
        "CreateSymbolicLinkA" => CreateSymbolicLinkA(emu),
        "CreateSymbolicLinkTransactedA" => CreateSymbolicLinkTransactedA(emu),
        "CreateSymbolicLinkTransactedW" => CreateSymbolicLinkTransactedW(emu),
        "CreateTapePartition" => CreateTapePartition(emu),
        "CreateWaitableTimerA" => CreateWaitableTimerA(emu),
        "CreateWaitableTimerExA" => CreateWaitableTimerExA(emu),
        "CreateWaitableTimerW" => CreateWaitableTimerW(emu),
        "DeactivateActCtxWorker" => DeactivateActCtxWorker(emu),
        "DebugSetProcessKillOnExit" => DebugSetProcessKillOnExit(emu),
        "DefineDosDeviceA" => DefineDosDeviceA(emu),
        "DeleteAtom" => DeleteAtom(emu),
        "DeleteFileTransactedW" => DeleteFileTransactedW(emu),
        "DeleteTimerQueue" => DeleteTimerQueue(emu),
        "DeleteVolumeMountPointA" => DeleteVolumeMountPointA(emu),
        "DeviceIoControlImplementation" => DeviceIoControlImplementation(emu),
        "DeviceNameCompare" => DeviceNameCompare(emu),
        "DisableThreadProfiling" => DisableThreadProfiling(emu),
        "DnsHostnameToComputerNameA" => DnsHostnameToComputerNameA(emu),
        "DnsHostnameToComputerNameW" => DnsHostnameToComputerNameW(emu),
        "DosDateTimeToFileTime" => DosDateTimeToFileTime(emu),
        "DosPathToSessionPathA" => DosPathToSessionPathA(emu),
        "DosPathToSessionPathW" => DosPathToSessionPathW(emu),
        "DuplicateEncryptionInfoFileExt" => DuplicateEncryptionInfoFileExt(emu),
        "EnableThreadProfiling" => EnableThreadProfiling(emu),
        "EndUpdateResourceA" => EndUpdateResourceA(emu),
        "EndUpdateResourceW" => EndUpdateResourceW(emu),
        "EnumCalendarInfoA" => EnumCalendarInfoA(emu),
        "EnumCalendarInfoExA" => EnumCalendarInfoExA(emu),
        "EnumDateFormatsA" => EnumDateFormatsA(emu),
        "EnumDateFormatsExA" => EnumDateFormatsExA(emu),
        "EnumLanguageGroupLocalesA" => EnumLanguageGroupLocalesA(emu),
        "EnumSystemCodePagesA" => EnumSystemCodePagesA(emu),
        "EnumSystemGeoID" => EnumSystemGeoID(emu),
        "EnumSystemGeoNames" => EnumSystemGeoNames(emu),
        "EnumSystemLanguageGroupsA" => EnumSystemLanguageGroupsA(emu),
        "EnumTimeFormatsA" => EnumTimeFormatsA(emu),
        "EnumUILanguagesA" => EnumUILanguagesA(emu),
        "EnumerateLocalComputerNamesA" => EnumerateLocalComputerNamesA(emu),
        "EnumerateLocalComputerNamesW" => EnumerateLocalComputerNamesW(emu),
        "EraseTape" => EraseTape(emu),
        "FatalExit" => FatalExit(emu),
        "FindActCtxSectionGuidWorker" => FindActCtxSectionGuidWorker(emu),
        "FindActCtxSectionStringA" => FindActCtxSectionStringA(emu),
        "FindActCtxSectionStringWWorker" => FindActCtxSectionStringWWorker(emu),
        "FindAtomA" => FindAtomA(emu),
        "FindAtomW" => FindAtomW(emu),
        "FindFirstVolumeA" => FindFirstVolumeA(emu),
        "FindFirstVolumeMountPointA" => FindFirstVolumeMountPointA(emu),
        "FindFirstVolumeMountPointW" => FindFirstVolumeMountPointW(emu),
        "FindNextVolumeA" => FindNextVolumeA(emu),
        "FindNextVolumeMountPointA" => FindNextVolumeMountPointA(emu),
        "FindNextVolumeMountPointW" => FindNextVolumeMountPointW(emu),
        "FindVolumeMountPointClose" => FindVolumeMountPointClose(emu),
        "FoldStringA" => FoldStringA(emu),
        "FreeMemoryJobObject" => FreeMemoryJobObject(emu),
        "GetActiveProcessorCount" => GetActiveProcessorCount(emu),
        "GetActiveProcessorGroupCount" => GetActiveProcessorGroupCount(emu),
        "GetApplicationRestartSettingsWorker" => GetApplicationRestartSettingsWorker(emu),
        "GetAtomNameA" => GetAtomNameA(emu),
        "GetAtomNameW" => GetAtomNameW(emu),
        "GetBinaryTypeW" => GetBinaryTypeW(emu),
        "GetCalendarDateFormat" => GetCalendarDateFormat(emu),
        "GetCalendarDateFormatEx" => GetCalendarDateFormatEx(emu),
        "GetCalendarDaysInMonth" => GetCalendarDaysInMonth(emu),
        "GetCalendarDifferenceInDays" => GetCalendarDifferenceInDays(emu),
        "GetCalendarInfoA" => GetCalendarInfoA(emu),
        "GetCalendarMonthsInYear" => GetCalendarMonthsInYear(emu),
        "GetCalendarSupportedDateRange" => GetCalendarSupportedDateRange(emu),
        "GetCalendarWeekNumber" => GetCalendarWeekNumber(emu),
        "GetComPlusPackageInstallStatus" => GetComPlusPackageInstallStatus(emu),
        "GetCompatFlags" => GetCompatFlags(emu),
        "GetCompressedFileSizeTransactedW" => GetCompressedFileSizeTransactedW(emu),
        "GetComputerNameW" => GetComputerNameW(emu),
        "GetConfigDialogName" => GetConfigDialogName(emu),
        "GetConsoleCharType" => GetConsoleCharType(emu),
        "GetConsoleFontInfo" => GetConsoleFontInfo(emu),
        "GetConsoleHardwareState" => GetConsoleHardwareState(emu),
        "GetConsoleInputWaitHandle" => GetConsoleInputWaitHandle(emu),
        "GetConsoleKeyboardLayoutNameA" => GetConsoleKeyboardLayoutNameA(emu),
        "GetConsoleKeyboardLayoutNameW" => GetConsoleKeyboardLayoutNameW(emu),
        "GetCorruptDetectionState" => GetCorruptDetectionState(emu),
        "GetCurrencyFormatA" => GetCurrencyFormatA(emu),
        "GetCurrencyFormatW" => GetCurrencyFormatW(emu),
        "GetCurrentActCtxWorker" => GetCurrentActCtxWorker(emu),
        "GetCurrentJaEraIndex" => GetCurrentJaEraIndex(emu),
        "GetCurrentThread" => GetCurrentThread(emu),
        "GetDateFormatAWorker" => GetDateFormatAWorker(emu),
        "GetDateFormatWWorker" => GetDateFormatWWorker(emu),
        "GetDefaultCommConfigA" => GetDefaultCommConfigA(emu),
        "GetDefaultCommConfigW" => GetDefaultCommConfigW(emu),
        "GetDevicePowerState" => GetDevicePowerState(emu),
        "GetDllDirectoryA" => GetDllDirectoryA(emu),
        "GetDllDirectoryW" => GetDllDirectoryW(emu),
        "GetDurationFormat" => GetDurationFormat(emu),
        "GetEncryptedFileVersionExt" => GetEncryptedFileVersionExt(emu),
        "GetExitCodeProcessImplementation" => GetExitCodeProcessImplementation(emu),
        "GetExpandedNameA" => GetExpandedNameA(emu),
        "GetExpandedNameW" => GetExpandedNameW(emu),
        "GetFileAttributesTransactedA" => GetFileAttributesTransactedA(emu),
        "GetFileBandwidthReservation" => GetFileBandwidthReservation(emu),
        "GetFirmwareEnvironmentVariableA" => GetFirmwareEnvironmentVariableA(emu),
        "GetFirmwareEnvironmentVariableW" => GetFirmwareEnvironmentVariableW(emu),
        "GetFirmwareType" => GetFirmwareType(emu),
        "GetFriendlyMatchComm" => GetFriendlyMatchComm(emu),
        "GetFullPathNameTransactedA" => GetFullPathNameTransactedA(emu),
        "GetFullPathNameTransactedW" => GetFullPathNameTransactedW(emu),
        "GetGeoInfoA" => GetGeoInfoA(emu),
        "GetGeoInfoEx" => GetGeoInfoEx(emu),
        "GetGeoInfoW" => GetGeoInfoW(emu),
        "GetHandleContext" => GetHandleContext(emu),
        "GetJapaneseEraItems" => GetJapaneseEraItems(emu),
        "GetKnownJapanEraCount" => GetKnownJapanEraCount(emu),
        "GetLongPathNameA" => GetLongPathNameA(emu),
        "GetMailslotInfo" => GetMailslotInfo(emu),
        "GetMaxJapanEras" => GetMaxJapanEras(emu),
        "GetMaxJapanYears" => GetMaxJapanYears(emu),
        "GetMaximumProcessorCount" => GetMaximumProcessorCount(emu),
        "GetMaximumProcessorGroupCount" => GetMaximumProcessorGroupCount(emu),
        "GetNamedPipeClientComputerNameA" => GetNamedPipeClientComputerNameA(emu),
        "GetNamedPipeClientProcessId" => GetNamedPipeClientProcessId(emu),
        "GetNamedPipeClientSessionId" => GetNamedPipeClientSessionId(emu),
        "GetNamedPipeHandleStateA" => GetNamedPipeHandleStateA(emu),
        "GetNamedPipeServerSessionId" => GetNamedPipeServerSessionId(emu),
        "GetNumaAvailableMemoryNode" => GetNumaAvailableMemoryNode(emu),
        "GetNumaAvailableMemoryNodeEx" => GetNumaAvailableMemoryNodeEx(emu),
        "GetNumaNodeNumberFromHandle" => GetNumaNodeNumberFromHandle(emu),
        "GetNumaNodeProcessorMask" => GetNumaNodeProcessorMask(emu),
        "GetNumaProcessorNode" => GetNumaProcessorNode(emu),
        "GetNumaProcessorNodeEx" => GetNumaProcessorNodeEx(emu),
        "GetNumaProximityNode" => GetNumaProximityNode(emu),
        "GetNumberFormatA" => GetNumberFormatA(emu),
        "GetNumberFormatW" => GetNumberFormatW(emu),
        "GetNumberOfConsoleFonts" => GetNumberOfConsoleFonts(emu),
        "GetPrivateProfileIntA" => GetPrivateProfileIntA(emu),
        "GetPrivateProfileIntW" => GetPrivateProfileIntW(emu),
        "GetPrivateProfileSectionA" => GetPrivateProfileSectionA(emu),
        "GetPrivateProfileSectionNamesA" => GetPrivateProfileSectionNamesA(emu),
        "GetPrivateProfileSectionNamesW" => GetPrivateProfileSectionNamesW(emu),
        "GetPrivateProfileSectionW" => GetPrivateProfileSectionW(emu),
        "GetPrivateProfileStringA" => GetPrivateProfileStringA(emu),
        "GetPrivateProfileStringW" => GetPrivateProfileStringW(emu),
        "GetPrivateProfileStructA" => GetPrivateProfileStructA(emu),
        "GetPrivateProfileStructW" => GetPrivateProfileStructW(emu),
        "GetProcessDEPPolicy" => GetProcessDEPPolicy(emu),
        "GetProcessIoCounters" => GetProcessIoCounters(emu),
        "GetProcessWorkingSetSize" => GetProcessWorkingSetSize(emu),
        "GetProfileIntA" => GetProfileIntA(emu),
        "GetProfileIntW" => GetProfileIntW(emu),
        "GetProfileSectionA" => GetProfileSectionA(emu),
        "GetProfileSectionW" => GetProfileSectionW(emu),
        "GetProfileStringA" => GetProfileStringA(emu),
        "GetProfileStringW" => GetProfileStringW(emu),
        "GetShortPathNameA" => GetShortPathNameA(emu),
        "GetShortPathNameW" => GetShortPathNameW(emu),
        "GetSystemDEPPolicy" => GetSystemDEPPolicy(emu),
        "GetSystemPowerStatus" => GetSystemPowerStatus(emu),
        "GetSystemRegistryQuota" => GetSystemRegistryQuota(emu),
        "GetTapeParameters" => GetTapeParameters(emu),
        "GetTapePosition" => GetTapePosition(emu),
        "GetTapeStatus" => GetTapeStatus(emu),
        "GetThreadSelectorEntry" => GetThreadSelectorEntry(emu),
        "GetTimeFormatWWorker" => GetTimeFormatWWorker(emu),
        "GetUserDefaultGeoName" => GetUserDefaultGeoName(emu),
        "GetUserGeoID" => GetUserGeoID(emu),
        "GetVolumeNameForVolumeMountPointA" => GetVolumeNameForVolumeMountPointA(emu),
        "GetVolumePathNamesForVolumeNameA" => GetVolumePathNamesForVolumeNameA(emu),
        "GlobalAddAtomExA" => GlobalAddAtomExA(emu),
        "GlobalAddAtomExW" => GlobalAddAtomExW(emu),
        "GlobalAddAtomW" => GlobalAddAtomW(emu),
        "GlobalDeleteAtom" => GlobalDeleteAtom(emu),
        "GlobalFindAtomA" => GlobalFindAtomA(emu),
        "GlobalFindAtomW" => GlobalFindAtomW(emu),
        "GlobalFix" => GlobalFix(emu),
        "GlobalFlags" => GlobalFlags(emu),
        "GlobalGetAtomNameA" => GlobalGetAtomNameA(emu),
        "GlobalGetAtomNameW" => GlobalGetAtomNameW(emu),
        "GlobalHandle" => GlobalHandle(emu),
        "GlobalLock" => GlobalLock(emu),
        "GlobalMemoryStatus" => GlobalMemoryStatus(emu),
        "GlobalReAlloc" => GlobalReAlloc(emu),
        "GlobalSize" => GlobalSize(emu),
        "GlobalUnWire" => GlobalUnWire(emu),
        "GlobalUnfix" => GlobalUnfix(emu),
        "GlobalUnlock" => GlobalUnlock(emu),
        "GlobalWire" => GlobalWire(emu),
        "Heap32First" => Heap32First(emu),
        "Heap32ListFirst" => Heap32ListFirst(emu),
        "Heap32ListNext" => Heap32ListNext(emu),
        "Heap32Next" => Heap32Next(emu),
        "InitAtomTable" => InitAtomTable(emu),
        "InitOnceGetStringTableOffset" => InitOnceGetStringTableOffset(emu),
        "Initialize16BitCriticalSection" => Initialize16BitCriticalSection(emu),
        "InterlockedCompareExchange" => InterlockedCompareExchange(emu),
        "InterlockedDecrement" => InterlockedDecrement(emu),
        "InterlockedExchange" => InterlockedExchange(emu),
        "InterlockedExchangeAdd" => InterlockedExchangeAdd(emu),
        "InternalCoStdMarshalObject" => InternalCoStdMarshalObject(emu),
        "InvalidateConsoleDIBits" => InvalidateConsoleDIBits(emu),
        "IsBadCodePtr" => IsBadCodePtr(emu),
        "IsBadHugeReadPtr" => IsBadHugeReadPtr(emu),
        "IsBadHugeWritePtr" => IsBadHugeWritePtr(emu),
        "IsBadReadPtr" => IsBadReadPtr(emu),
        "IsBadStringPtrA" => IsBadStringPtrA(emu),
        "IsBadStringPtrW" => IsBadStringPtrW(emu),
        "IsBadWritePtr" => IsBadWritePtr(emu),
        "IsCalendarLeapDay" => IsCalendarLeapDay(emu),
        "IsCalendarLeapMonth" => IsCalendarLeapMonth(emu),
        "IsCalendarLeapYear" => IsCalendarLeapYear(emu),
        "IsCallerAdminOrSystem" => IsCallerAdminOrSystem(emu),
        "IsFusionFullySupported" => IsFusionFullySupported(emu),
        "IsOOBESupported" => IsOOBESupported(emu),
        "IsSystemLUID" => IsSystemLUID(emu),
        "IsSystemResumeAutomatic" => IsSystemResumeAutomatic(emu),
        "IsTerminalServerCompatible" => IsTerminalServerCompatible(emu),
        "IsThreadAFiber" => IsThreadAFiber(emu),
        "IsValidCalDateTime" => IsValidCalDateTime(emu),
        "LoadAppInitDllsImplementation" => LoadAppInitDllsImplementation(emu),
        "LoadStringBaseW" => LoadStringBaseW(emu),
        "LocalCompact" => LocalCompact(emu),
        "LocalFlags" => LocalFlags(emu),
        "LocalHandle" => LocalHandle(emu),
        "LocalShrink" => LocalShrink(emu),
        "LocalSize" => LocalSize(emu),
        "lstrcmpA" => lstrcmpA(emu),
        "lstrcmp" => lstrcmpA(emu),
        "lstrcmpW" => lstrcmpW(emu),
        "lstrcmpiA" => lstrcmpiA(emu),
        "LZClose" => LZClose(emu),
        "LZCopy" => LZCopy(emu),
        "LZInit" => LZInit(emu),
        "LZOpenFileA" => LZOpenFileA(emu),
        "LZOpenFileW" => LZOpenFileW(emu),
        "LZRead" => LZRead(emu),
        "LZSeek" => LZSeek(emu),
        "LZStart" => LZStart(emu),
        "MachinePrefersNumericFirstYear" => MachinePrefersNumericFirstYear(emu),
        "MoveFileExA" => MoveFileExA(emu),
        "MoveFileTransactedA" => MoveFileTransactedA(emu),
        "MoveFileTransactedW" => MoveFileTransactedW(emu),
        "MulDiv" => MulDiv(emu),
        "NonInstallerDefault" => NonInstallerDefault(emu),
        "NotifyUILanguageChange" => NotifyUILanguageChange(emu),
        "NtVdm64CreateProcessInternalW" => NtVdm64CreateProcessInternalW(emu),
        "NtWow64ConsoleLaunchServerProcess" => NtWow64ConsoleLaunchServerProcess(emu),
        "NtWow64CsrBaseCheckRunApp" => NtWow64CsrBaseCheckRunApp(emu),
        "NtWow64CsrBasepCreateActCtx" => NtWow64CsrBasepCreateActCtx(emu),
        "NtWow64CsrBasepCreateProcess" => NtWow64CsrBasepCreateProcess(emu),
        "NtWow64CsrBasepDefineDosDevice" => NtWow64CsrBasepDefineDosDevice(emu),
        "NtWow64CsrBasepGetProcessShutdownParam" => NtWow64CsrBasepGetProcessShutdownParam(emu),
        "NtWow64CsrBasepNlsGetUserInfo" => NtWow64CsrBasepNlsGetUserInfo(emu),
        "NtWow64CsrBasepNlsUpdateCacheCount" => NtWow64CsrBasepNlsUpdateCacheCount(emu),
        "NtWow64CsrBasepRefreshIniFileMapping" => NtWow64CsrBasepRefreshIniFileMapping(emu),
        "NtWow64CsrBasepSetClientTimeZoneInformation" => {
            NtWow64CsrBasepSetClientTimeZoneInformation(emu)
        }
        "NtWow64CsrBasepSetProcessShutdownParam" => NtWow64CsrBasepSetProcessShutdownParam(emu),
        "NtWow64CsrBasepSoundSentryNotification" => NtWow64CsrBasepSoundSentryNotification(emu),
        "OOBEComplete" => OOBEComplete(emu),
        "OOBECompleteWnfCallback" => OOBECompleteWnfCallback(emu),
        "OOBECompleteWnfQueryCallback" => OOBECompleteWnfQueryCallback(emu),
        "OOBECompleteWnfWaitCallback" => OOBECompleteWnfWaitCallback(emu),
        "OpenFile" => OpenFile(emu),
        "OpenFileMappingA" => OpenFileMappingA(emu),
        "OpenJobObjectA" => OpenJobObjectA(emu),
        "OpenJobObjectW" => OpenJobObjectW(emu),
        "OpenMutexA" => OpenMutexA(emu),
        "OpenPrivateNamespaceA" => OpenPrivateNamespaceA(emu),
        "OpenSortIdKey" => OpenSortIdKey(emu),
        "PowerClearRequest" => PowerClearRequest(emu),
        "PowerCreateRequest" => PowerCreateRequest(emu),
        "PowerSetRequest" => PowerSetRequest(emu),
        "PrepareTape" => PrepareTape(emu),
        "PrivMoveFileIdentityW" => PrivMoveFileIdentityW(emu),
        "Process32FirstW" => Process32FirstW(emu),
        "Process32Next" => Process32Next(emu),
        "Process32NextW" => Process32NextW(emu),
        "QueryActCtxSettingsWWorker" => QueryActCtxSettingsWWorker(emu),
        "QueryActCtxWWorker" => QueryActCtxWWorker(emu),
        "QueryDosDeviceA" => QueryDosDeviceA(emu),
        "QueryInformationJobObject" => QueryInformationJobObject(emu),
        "QueryIoRateControlInformationJobObject" => QueryIoRateControlInformationJobObject(emu),
        "QueryThreadProfiling" => QueryThreadProfiling(emu),
        "QuirkGetDataWorker" => QuirkGetDataWorker(emu),
        "QuirkIsEnabled2Worker" => QuirkIsEnabled2Worker(emu),
        "QuirkIsEnabledForPackage2Worker" => QuirkIsEnabledForPackage2Worker(emu),
        "QuirkIsEnabledForPackage3Worker" => QuirkIsEnabledForPackage3Worker(emu),
        "QuirkIsEnabledForPackage4Worker" => QuirkIsEnabledForPackage4Worker(emu),
        "QuirkIsEnabledForPackageWorker" => QuirkIsEnabledForPackageWorker(emu),
        "QuirkIsEnabledForProcessWorker" => QuirkIsEnabledForProcessWorker(emu),
        "QuirkIsEnabledWorker" => QuirkIsEnabledWorker(emu),
        "RaiseInvalid16BitExeError" => RaiseInvalid16BitExeError(emu),
        "ReadThreadProfilingData" => ReadThreadProfilingData(emu),
        "RegisterApplicationRecoveryCallback" => RegisterApplicationRecoveryCallback(emu),
        "RegisterApplicationRestart" => RegisterApplicationRestart(emu),
        "RegisterConsoleIME" => RegisterConsoleIME(emu),
        "RegisterConsoleOS2" => RegisterConsoleOS2(emu),
        "RegisterConsoleVDM" => RegisterConsoleVDM(emu),
        "RegisterWaitForInputIdle" => RegisterWaitForInputIdle(emu),
        "RegisterWaitUntilOOBECompleted" => RegisterWaitUntilOOBECompleted(emu),
        "RemoveDirectoryTransactedW" => RemoveDirectoryTransactedW(emu),
        "RemoveLocalAlternateComputerNameW" => RemoveLocalAlternateComputerNameW(emu),
        "RemoveSecureMemoryCacheCallback" => RemoveSecureMemoryCacheCallback(emu),
        "ReplaceFileA" => ReplaceFileA(emu),
        "ReplacePartitionUnit" => ReplacePartitionUnit(emu),
        "RequestWakeupLatency" => RequestWakeupLatency(emu),
        "ResourceDataMatch" => ResourceDataMatch(emu),
        "ReturnMem16Data" => ReturnMem16Data(emu),
        "SbpIsTraceEnabled" => SbpIsTraceEnabled(emu),
        "SdbpCheckFromVersion" => SdbpCheckFromVersion(emu),
        "SdbpCheckMatchingDevice" => SdbpCheckMatchingDevice(emu),
        "SdbpCheckMatchingDir" => SdbpCheckMatchingDir(emu),
        "SdbpCheckMatchingFiles" => SdbpCheckMatchingFiles(emu),
        "SdbpCheckMatchingRegistry" => SdbpCheckMatchingRegistry(emu),
        "SdbpCheckMatchingText" => SdbpCheckMatchingText(emu),
        "SdbpCheckMatchingWildcardFiles" => SdbpCheckMatchingWildcardFiles(emu),
        "SdbpCheckMatchingWildcardRegistry" => SdbpCheckMatchingWildcardRegistry(emu),
        "SdbpCheckOSKind" => SdbpCheckOSKind(emu),
        "SdbpCheckPackageAttributes" => SdbpCheckPackageAttributes(emu),
        "SdbpCheckRuntimePlatform" => SdbpCheckRuntimePlatform(emu),
        "SdbpCheckUptoVersion" => SdbpCheckUptoVersion(emu),
        "SdbpCheckVersion" => SdbpCheckVersion(emu),
        "SdbpGetPathAppPatch" => SdbpGetPathAppPatch(emu),
        "SdbpGetPathAppPatchPreRS3" => SdbpGetPathAppPatchPreRS3(emu),
        "SdbpGetPathAppraiser" => SdbpGetPathAppraiser(emu),
        "SdbpGetPathCustomSdb" => SdbpGetPathCustomSdb(emu),
        "SdbpGetPathCustomSdbPreRS3" => SdbpGetPathCustomSdbPreRS3(emu),
        "SdbpGetPathSystem" => SdbpGetPathSystem(emu),
        "SetCalendarInfoA" => SetCalendarInfoA(emu),
        "SetComPlusPackageInstallStatus" => SetComPlusPackageInstallStatus(emu),
        "SetConsoleCursor" => SetConsoleCursor(emu),
        "SetConsoleFont" => SetConsoleFont(emu),
        "SetConsoleHardwareState" => SetConsoleHardwareState(emu),
        "SetConsoleIcon" => SetConsoleIcon(emu),
        "SetConsoleKeyShortcuts" => SetConsoleKeyShortcuts(emu),
        "SetConsoleLocalEUDC" => SetConsoleLocalEUDC(emu),
        "SetConsoleMaximumWindowSize" => SetConsoleMaximumWindowSize(emu),
        "SetConsoleMenuClose" => SetConsoleMenuClose(emu),
        "SetConsoleOS2OemFormat" => SetConsoleOS2OemFormat(emu),
        "SetConsolePalette" => SetConsolePalette(emu),
        "SetDefaultCommConfigA" => SetDefaultCommConfigA(emu),
        "SetDllDirectoryA" => SetDllDirectoryA(emu),
        "SetDllDirectoryW" => SetDllDirectoryW(emu),
        "SetEnvironmentStringsA" => SetEnvironmentStringsA(emu),
        "SetFileAttributesTransactedW" => SetFileAttributesTransactedW(emu),
        "SetFileBandwidthReservation" => SetFileBandwidthReservation(emu),
        "SetFileShortNameA" => SetFileShortNameA(emu),
        "SetFileShortNameW" => SetFileShortNameW(emu),
        "SetFirmwareEnvironmentVariableA" => SetFirmwareEnvironmentVariableA(emu),
        "SetFirmwareEnvironmentVariableW" => SetFirmwareEnvironmentVariableW(emu),
        "SetInformationJobObject" => SetInformationJobObject(emu),
        "SetIoRateControlInformationJobObject" => SetIoRateControlInformationJobObject(emu),
        "SetLocalPrimaryComputerNameW" => SetLocalPrimaryComputerNameW(emu),
        "SetLocaleInfoA" => SetLocaleInfoA(emu),
        "SetMailslotInfo" => SetMailslotInfo(emu),
        "SetMessageWaitingIndicator" => SetMessageWaitingIndicator(emu),
        "SetNamedPipeAttribute" => SetNamedPipeAttribute(emu),
        "SetProcessAffinityMask" => SetProcessAffinityMask(emu),
        "SetProcessDEPPolicy" => SetProcessDEPPolicy(emu),
        "SetProcessWorkingSetSize" => SetProcessWorkingSetSize(emu),
        "SetSystemPowerState" => SetSystemPowerState(emu),
        "SetSystemTimeAdjustment" => SetSystemTimeAdjustment(emu),
        "SetTapeParameters" => SetTapeParameters(emu),
        "SetTapePosition" => SetTapePosition(emu),
        "SetThreadAffinityMask" => SetThreadAffinityMask(emu),
        "SetThreadExecutionState" => SetThreadExecutionState(emu),
        "SetTimerQueueTimer" => SetTimerQueueTimer(emu),
        "SetUserGeoID" => SetUserGeoID(emu),
        "SetUserGeoName" => SetUserGeoName(emu),
        "SetVolumeLabelA" => SetVolumeLabelA(emu),
        "SetVolumeLabelW" => SetVolumeLabelW(emu),
        "SetVolumeMountPointA" => SetVolumeMountPointA(emu),
        "SetVolumeMountPointW" => SetVolumeMountPointW(emu),
        "ShowConsoleCursor" => ShowConsoleCursor(emu),
        "SortChangeCase" => SortChangeCase(emu),
        "SortCompareString" => SortCompareString(emu),
        "SortFindString" => SortFindString(emu),
        "SortGetHashCode" => SortGetHashCode(emu),
        "SortIsDefinedString" => SortIsDefinedString(emu),
        "StringCbCatExW" => StringCbCatExW(emu),
        "StringCbCopyExW" => StringCbCopyExW(emu),
        "StringCbCopyW" => StringCbCopyW(emu),
        "StringCchCatW" => StringCchCatW(emu),
        "StringCchCopyExW" => StringCchCopyExW(emu),
        "StringCchCopyNW" => StringCchCopyNW(emu),
        "StringCchCopyW" => StringCchCopyW(emu),
        "StringCchLengthW" => StringCchLengthW(emu),
        "StringCopyWorkerA" => StringCopyWorkerA(emu),
        "StringCopyWorkerW" => StringCopyWorkerW(emu),
        "StringCopyWorkerW_2_0" => StringCopyWorkerW_2_0(emu),
        "StringExValidateDestW" => StringExValidateDestW(emu),
        "StringLengthWorkerA" => StringLengthWorkerA(emu),
        "StringLengthWorkerW" => StringLengthWorkerW(emu),
        "StringTable" => StringTable(emu),
        "StringVPrintfWorkerW" => StringVPrintfWorkerW(emu),
        "StringValidateDestAndLengthW" => StringValidateDestAndLengthW(emu),
        "SXSManifest" => SXSManifest(emu),
        "TermsrvConvertSysRootToUserDir" => TermsrvConvertSysRootToUserDir(emu),
        "TermsrvCreateRegEntry" => TermsrvCreateRegEntry(emu),
        "TermsrvDeleteKey" => TermsrvDeleteKey(emu),
        "TermsrvDeleteValue" => TermsrvDeleteValue(emu),
        "TermsrvGetPreSetValue" => TermsrvGetPreSetValue(emu),
        "TermsrvOpenRegEntry" => TermsrvOpenRegEntry(emu),
        "TermsrvOpenUserClasses" => TermsrvOpenUserClasses(emu),
        "TermsrvRestoreKey" => TermsrvRestoreKey(emu),
        "TermsrvSetValueKey" => TermsrvSetValueKey(emu),
        "TermsrvSyncUserIniFile" => TermsrvSyncUserIniFile(emu),
        "TermsrvSyncUserIniFileExt" => TermsrvSyncUserIniFileExt(emu),
        "timeBeginPeriod" => timeEndPeriod(emu),
        "timeEndPeriod" => timeEndPeriod(emu),
        "timeGetDevCaps" => timeGetDevCaps(emu),
        "timeGetTime" => timeGetTime(emu),
        "TimeInit" => TimeInit(emu),
        "TraceLoggingRegisterEx_EventRegister_EventSetInformation" => {
            TraceLoggingRegisterEx_EventRegister_EventSetInformation(emu)
        }
        "ULongAdd" => ULongAdd(emu),
        "ULongLongToULong" => ULongLongToULong(emu),
        "ULongMult" => ULongMult(emu),
        "UnregisterApplicationRecoveryCallback" => UnregisterApplicationRecoveryCallback(emu),
        "UnregisterApplicationRestart" => UnregisterApplicationRestart(emu),
        "UnregisterWait" => UnregisterWait(emu),
        "UnregisterWaitUntilOOBECompleted" => UnregisterWaitUntilOOBECompleted(emu),
        "UpdateCalendarDayOfWeek" => UpdateCalendarDayOfWeek(emu),
        "UpdateResourceA" => UpdateResourceA(emu),
        "UpdateResourceW" => UpdateResourceW(emu),
        "VDMConsoleOperation" => VDMConsoleOperation(emu),
        "VDMOperationStarted" => VDMOperationStarted(emu),
        "VerifyConsoleIoHandle" => VerifyConsoleIoHandle(emu),
        "VerifyVersionInfoA" => VerifyVersionInfoA(emu),
        "WaitNamedPipeA" => WaitNamedPipeA(emu),
        "WerGetFlagsWorker" => WerGetFlagsWorker(emu),
        "WerRegisterFileWorker" => WerRegisterFileWorker(emu),
        "WerSetFlags" => WerSetFlags(emu),
        "WerUnregisterFileWorker" => WerUnregisterFileWorker(emu),
        "WerUnregisterMemoryBlockWorker" => WerUnregisterMemoryBlockWorker(emu),
        "WerpAcquirePebLock" => WerpAcquirePebLock(emu),
        "WerpCheckOkToRegister" => WerpCheckOkToRegister(emu),
        "WerpGetDebugger" => WerpGetDebugger(emu),
        "WerpInitPEBStore" => WerpInitPEBStore(emu),
        "WerpInitiateRemoteRecovery" => WerpInitiateRemoteRecovery(emu),
        "WerpLaunchAeDebug" => WerpLaunchAeDebug(emu),
        "WerpMapProtectionLevel" => WerpMapProtectionLevel(emu),
        "WerpNotifyUseStringResourceWorker" => WerpNotifyUseStringResourceWorker(emu),
        "WerpRecoveryInvokedRemotely" => WerpRecoveryInvokedRemotely(emu),
        "Wow64EnableWow64FsRedirection" => Wow64EnableWow64FsRedirection(emu),
        "Wow64GetThreadSelectorEntry" => Wow64GetThreadSelectorEntry(emu),
        "Wow64SuspendThread" => Wow64SuspendThread(emu),
        "Wow64SystemServiceCall" => Wow64SystemServiceCall(emu),
        "Wow64TransitionResolve" => Wow64TransitionResolve(emu),
        "WriteConsoleInputVDMA" => WriteConsoleInputVDMA(emu),
        "WriteConsoleInputVDMW" => WriteConsoleInputVDMW(emu),
        "WritePrivateProfileSectionA" => WritePrivateProfileSectionA(emu),
        "WritePrivateProfileSectionW" => WritePrivateProfileSectionW(emu),
        "WritePrivateProfileStringA" => WritePrivateProfileStringA(emu),
        "WritePrivateProfileStringW" => WritePrivateProfileStringW(emu),
        "WritePrivateProfileStructA" => WritePrivateProfileStructA(emu),
        "WritePrivateProfileStructW" => WritePrivateProfileStructW(emu),
        "WriteProfileSectionA" => WriteProfileSectionA(emu),
        "WriteProfileSectionW" => WriteProfileSectionW(emu),
        "WriteProfileStringA" => WriteProfileStringA(emu),
        "WriteProfileStringW" => WriteProfileStringW(emu),
        "WriteTapemark" => WriteTapemark(emu),
        "ZombifyActCtxWorker" => ZombifyActCtxWorker(emu),

        // ---- auto-generated stubs from kernel32.dll export table ----
        "ActivateActCtx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ActivateActCtx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ActivatePackageVirtualizationContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ActivatePackageVirtualizationContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddConsoleAliasA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!AddConsoleAliasA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AddConsoleAliasW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!AddConsoleAliasW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AddDllDirectory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AddDllDirectory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddLocalAlternateComputerNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!AddLocalAlternateComputerNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AddRefActCtx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AddRefActCtx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddRefActCtxWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AddRefActCtxWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddResourceAttributeAce" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AddResourceAttributeAce arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddSIDToBoundaryDescriptor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AddSIDToBoundaryDescriptor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddScopedPolicyIDAce" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AddScopedPolicyIDAce arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddSecureMemoryCacheCallback" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AddSecureMemoryCacheCallback arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddVectoredContinueHandler" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AddVectoredContinueHandler h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AllocConsole" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!AllocConsole arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            let handle = helper::handler_create("allocconsole");
            emu.regs_mut().rax = handle;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "AllocConsoleWithOptions" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AllocConsoleWithOptions arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AllocateUserPhysicalPages" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AllocateUserPhysicalPages arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AllocateUserPhysicalPagesNuma" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AllocateUserPhysicalPagesNuma arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppPolicyGetClrCompat" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppPolicyGetClrCompat arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppPolicyGetCreateFileAccess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppPolicyGetCreateFileAccess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppPolicyGetLifecycleManagement" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppPolicyGetLifecycleManagement arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppPolicyGetMediaFoundationCodecLoading" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppPolicyGetMediaFoundationCodecLoading arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppPolicyGetProcessTerminationMethod" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppPolicyGetProcessTerminationMethod h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppPolicyGetShowDeveloperDiagnostic" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppPolicyGetShowDeveloperDiagnostic arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppPolicyGetThreadInitializationType" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppPolicyGetThreadInitializationType h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppPolicyGetWindowingModel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppPolicyGetWindowingModel arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppXGetOSMaxVersionTested" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AppXGetOSMaxVersionTested arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AreShortNamesEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AreShortNamesEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AssignProcessToJobObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!AssignProcessToJobObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AttachConsole" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!AttachConsole");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BackupReadEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BackupReadEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BackupWriteEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BackupWriteEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseCheckAppcompatCache" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseCheckAppcompatCache arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseCheckAppcompatCacheEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseCheckAppcompatCacheEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseCleanupAppcompatCacheSupport" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseCleanupAppcompatCacheSupport arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseDumpAppcompatCache" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseDumpAppcompatCache arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseDumpAppcompatCacheWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseDumpAppcompatCacheWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseElevationPostProcessing" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseElevationPostProcessing h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseFlushAppcompatCache" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseFlushAppcompatCache arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseFormatObjectAttributes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseFormatObjectAttributes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseFormatTimeOut" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseFormatTimeOut arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseGetNamedObjectDirectory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseGetNamedObjectDirectory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseInitAppcompatCacheSupport" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseInitAppcompatCacheSupport arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseInitAppcompatCacheSupportWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseInitAppcompatCacheSupportWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseIsAppcompatInfrastructureDisabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseIsAppcompatInfrastructureDisabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseIsAppcompatInfrastructureDisabledWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseIsAppcompatInfrastructureDisabledWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseQueryModuleData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseQueryModuleData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseThreadInitThunk" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseThreadInitThunk h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BaseUpdateAppcompatCache" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BaseUpdateAppcompatCache arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepCheckPplSupport" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepCheckPplSupport arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepFinishPackageActivation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepFinishPackageActivation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepFinishPackageActivationForSxS" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepFinishPackageActivationForSxS arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepFreeActivationTokenInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepFreeActivationTokenInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepGetAppCompatData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepGetAppCompatData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepGetPackageActivationTokenForFilePath" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepGetPackageActivationTokenForFilePath h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepGetPackageActivationTokenForFilePath2" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepGetPackageActivationTokenForFilePath2 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepGetPackageActivationTokenForSxS" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepGetPackageActivationTokenForSxS h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepGetPackageActivationTokenForSxS2" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepGetPackageActivationTokenForSxS2 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepGetPackageActivationTokenForSxS3" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepGetPackageActivationTokenForSxS3 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepGetPackagedAppInfoForFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepGetPackagedAppInfoForFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepMapModuleHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepMapModuleHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepQueryModuleChpeSettings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepQueryModuleChpeSettings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepReleaseAppXContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepReleaseAppXContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BasepReleasePackagedAppInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BasepReleasePackagedAppInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Beep" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!Beep arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildIoRingCancelRequest" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BuildIoRingCancelRequest arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildIoRingFlushFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BuildIoRingFlushFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildIoRingReadFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BuildIoRingReadFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildIoRingReadFileScatter" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BuildIoRingReadFileScatter h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildIoRingRegisterBuffers" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BuildIoRingRegisterBuffers arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildIoRingRegisterFileHandles" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BuildIoRingRegisterFileHandles h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildIoRingWriteFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BuildIoRingWriteFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildIoRingWriteFileGather" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!BuildIoRingWriteFileGather h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallNamedPipeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!CallNamedPipeW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CallbackMayRunLong" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CallbackMayRunLong arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CancelDeviceWakeupRequest" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CancelDeviceWakeupRequest h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CancelIo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!CancelIo h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CancelIoEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CancelIoEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CancelSynchronousIo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CancelSynchronousIo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CancelThreadpoolIo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CancelThreadpoolIo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CancelWaitableTimer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!CancelWaitableTimer 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CeipIsOptedIn" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CeipIsOptedIn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeTimerQueueTimer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!ChangeTimerQueueTimer 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CheckAllowDecryptedRemoteDestinationPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CheckAllowDecryptedRemoteDestinationPolicy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckElevationEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CheckElevationEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckForReadOnlyResourceFilter" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CheckForReadOnlyResourceFilter arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckRemoteDebuggerPresent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CheckRemoteDebuggerPresent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckTokenCapability" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CheckTokenCapability h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckTokenMembershipEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CheckTokenMembershipEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClearCommBreak" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ClearCommBreak arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClearCommError" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ClearCommError arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseIoRing" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CloseIoRing h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClosePackageInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ClosePackageInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClosePrivateNamespace" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ClosePrivateNamespace h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseProfileUserMapping" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CloseProfileUserMapping h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClosePseudoConsole" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ClosePseudoConsole h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseState" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CloseState h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseThreadpool" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CloseThreadpool h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseThreadpoolCleanupGroup" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CloseThreadpoolCleanupGroup h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseThreadpoolCleanupGroupMembers" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CloseThreadpoolCleanupGroupMembers h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseThreadpoolIo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CloseThreadpoolIo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseThreadpoolTimer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!CloseThreadpoolTimer 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseThreadpoolWait" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!CloseThreadpoolWait 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseThreadpoolWork" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CloseThreadpoolWork h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CmdBatNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CmdBatNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CommConfigDialogW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!CommConfigDialogW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CompareFileTime" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!CompareFileTime p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CompareStringA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!CompareStringA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 2;
        }
        "CompareStringEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CompareStringEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CompareStringOrdinal" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CompareStringOrdinal p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
        }
        "ContinueDebugEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!ContinueDebugEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ConvertDefaultLocale" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ConvertDefaultLocale arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ConvertFiberToThread" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ConvertFiberToThread arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ConvertThreadToFiber" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ConvertThreadToFiber arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ConvertThreadToFiberEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ConvertThreadToFiberEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CopyContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CopyContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CopyFile2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CopyFile2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CopyFileExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!CopyFileExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateActCtxW" => {
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateActCtxW");
            emu.regs_mut().rax = helper::handler_create("CreateActCtxW");
        }
        "CreateBoundaryDescriptorW" => {
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateBoundaryDescriptorW");
            emu.regs_mut().rax = helper::handler_create("CreateBoundaryDescriptorW");
        }
        "CreateConsoleScreenBuffer" => {
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateConsoleScreenBuffer");
            emu.regs_mut().rax = helper::handler_create("CreateConsoleScreenBuffer");
        }
        "CreateDirectory2A" => {
            log_red!(emu, "kernel32!CreateDirectory2A");
            emu.regs_mut().rax = helper::handler_create("CreateDirectory2A");
        }
        "CreateDirectory2W" => {
            log_red!(emu, "kernel32!CreateDirectory2W");
            emu.regs_mut().rax = helper::handler_create("CreateDirectory2W");
        }
        "CreateDirectoryExW" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateDirectoryExW");
            emu.regs_mut().rax = helper::handler_create("CreateDirectoryExW");
        }
        "CreateEnclave" => {
            log_red!(emu, "kernel32!CreateEnclave");
            emu.regs_mut().rax = helper::handler_create("CreateEnclave");
        }
        "CreateEventExA" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateEventExA");
            emu.regs_mut().rax = helper::handler_create("CreateEventExA");
        }
        "CreateEventExW" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateEventExW");
            emu.regs_mut().rax = helper::handler_create("CreateEventExW");
        }
        "CreateEventW" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateEventW");
            emu.regs_mut().rax = helper::handler_create("CreateEventW");
        }
        "CreateFiber" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateFiber");
            emu.regs_mut().rax = helper::handler_create("CreateFiber");
        }
        "CreateFiberEx" => {
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateFiberEx");
            emu.regs_mut().rax = helper::handler_create("CreateFiberEx");
        }
        "CreateFile2" => {
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateFile2");
            emu.regs_mut().rax = helper::handler_create("CreateFile2");
        }
        "CreateFile3" => {
            log_red!(emu, "kernel32!CreateFile3");
            emu.regs_mut().rax = helper::handler_create("CreateFile3");
        }
        "CreateFileMappingFromApp" => {
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateFileMappingFromApp");
            emu.regs_mut().rax = helper::handler_create("CreateFileMappingFromApp");
        }
        "CreateFileMappingNumaW" => {
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateFileMappingNumaW");
            emu.regs_mut().rax = helper::handler_create("CreateFileMappingNumaW");
        }
        "CreateFileTransactedA" => {
            for _ in 0..10u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateFileTransactedA");
            emu.regs_mut().rax = helper::handler_create("CreateFileTransactedA");
        }
        "CreateFileTransactedW" => {
            for _ in 0..10u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateFileTransactedW");
            emu.regs_mut().rax = helper::handler_create("CreateFileTransactedW");
        }
        "CreateHardLinkA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!CreateHardLinkA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateHardLinkTransactedW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!CreateHardLinkTransactedW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateHardLinkW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!CreateHardLinkW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateIoCompletionPort" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateIoCompletionPort");
            emu.regs_mut().rax = helper::handler_create("CreateIoCompletionPort");
        }
        "CreateIoRing" => {
            log_red!(emu, "kernel32!CreateIoRing");
            emu.regs_mut().rax = helper::handler_create("CreateIoRing");
        }
        "CreateJobObjectA" => {
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateJobObjectA");
            emu.regs_mut().rax = helper::handler_create("CreateJobObjectA");
        }
        "CreateJobObjectW" => {
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateJobObjectW");
            emu.regs_mut().rax = helper::handler_create("CreateJobObjectW");
        }
        "CreateJobSet" => {
            log_red!(emu, "kernel32!CreateJobSet");
            emu.regs_mut().rax = helper::handler_create("CreateJobSet");
        }
        "CreateMailslotA" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateMailslotA");
            emu.regs_mut().rax = helper::handler_create("CreateMailslotA");
        }
        "CreateMailslotW" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateMailslotW");
            emu.regs_mut().rax = helper::handler_create("CreateMailslotW");
        }
        "CreateMemoryResourceNotification" => {
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateMemoryResourceNotification");
            emu.regs_mut().rax = helper::handler_create("CreateMemoryResourceNotification");
        }
        "CreateMutexExA" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateMutexExA");
            emu.regs_mut().rax = helper::handler_create("CreateMutexExA");
        }
        "CreateMutexExW" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateMutexExW");
            emu.regs_mut().rax = helper::handler_create("CreateMutexExW");
        }
        "CreateNamedPipeW" => {
            for _ in 0..8u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateNamedPipeW");
            emu.regs_mut().rax = helper::handler_create("CreateNamedPipeW");
        }
        "CreatePackageVirtualizationContext" => {
            log_red!(emu, "kernel32!CreatePackageVirtualizationContext");
            emu.regs_mut().rax = helper::handler_create("CreatePackageVirtualizationContext");
        }
        "CreatePipe" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreatePipe");
            emu.regs_mut().rax = helper::handler_create("CreatePipe");
        }
        "CreatePrivateNamespaceA" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreatePrivateNamespaceA");
            emu.regs_mut().rax = helper::handler_create("CreatePrivateNamespaceA");
        }
        "CreatePrivateNamespaceW" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreatePrivateNamespaceW");
            emu.regs_mut().rax = helper::handler_create("CreatePrivateNamespaceW");
        }
        "CreateProcessAsUserA" => {
            for _ in 0..11u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateProcessAsUserA");
            emu.regs_mut().rax = helper::handler_create("CreateProcessAsUserA");
        }
        "CreateProcessAsUserW" => {
            for _ in 0..11u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateProcessAsUserW");
            emu.regs_mut().rax = helper::handler_create("CreateProcessAsUserW");
        }
        "CreateProcessInternalA" => {
            log_red!(emu, "kernel32!CreateProcessInternalA");
            emu.regs_mut().rax = helper::handler_create("CreateProcessInternalA");
        }
        "CreateProcessInternalW" => {
            log_red!(emu, "kernel32!CreateProcessInternalW");
            emu.regs_mut().rax = helper::handler_create("CreateProcessInternalW");
        }
        "CreatePseudoConsole" => {
            log_red!(emu, "kernel32!CreatePseudoConsole");
            emu.regs_mut().rax = helper::handler_create("CreatePseudoConsole");
        }
        "CreateRemoteThreadEx" => {
            for _ in 0..8u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateRemoteThreadEx");
            emu.regs_mut().rax = helper::handler_create("CreateRemoteThreadEx");
        }
        "CreateSemaphoreExW" => {
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateSemaphoreExW");
            emu.regs_mut().rax = helper::handler_create("CreateSemaphoreExW");
        }
        "CreateSemaphoreW" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateSemaphoreW");
            emu.regs_mut().rax = helper::handler_create("CreateSemaphoreW");
        }
        "CreateSymbolicLinkW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!CreateSymbolicLinkW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateThreadpool" => {
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateThreadpool");
            emu.regs_mut().rax = helper::handler_create("CreateThreadpool");
        }
        "CreateThreadpoolCleanupGroup" => {
            log_red!(emu, "kernel32!CreateThreadpoolCleanupGroup");
            emu.regs_mut().rax = helper::handler_create("CreateThreadpoolCleanupGroup");
        }
        "CreateThreadpoolIo" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateThreadpoolIo");
            emu.regs_mut().rax = helper::handler_create("CreateThreadpoolIo");
        }
        "CreateThreadpoolTimer" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateThreadpoolTimer");
            emu.regs_mut().rax = helper::handler_create("CreateThreadpoolTimer");
        }
        "CreateThreadpoolWait" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateThreadpoolWait");
            emu.regs_mut().rax = helper::handler_create("CreateThreadpoolWait");
        }
        "CreateThreadpoolWork" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateThreadpoolWork");
            emu.regs_mut().rax = helper::handler_create("CreateThreadpoolWork");
        }
        "CreateTimerQueue" => {
            log_red!(emu, "kernel32!CreateTimerQueue");
            emu.regs_mut().rax = helper::handler_create("CreateTimerQueue");
        }
        "CreateTimerQueueTimer" => {
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateTimerQueueTimer");
            emu.regs_mut().rax = helper::handler_create("CreateTimerQueueTimer");
        }
        "CreateWaitableTimerExW" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!CreateWaitableTimerExW");
            emu.regs_mut().rax = helper::handler_create("CreateWaitableTimerExW");
        }
        "CtrlRoutine" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!CtrlRoutine arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeactivateActCtx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DeactivateActCtx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeactivatePackageVirtualizationContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DeactivatePackageVirtualizationContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DebugActiveProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DebugActiveProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DebugActiveProcessStop" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DebugActiveProcessStop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DebugBreak" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DebugBreak arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DebugBreakProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DebugBreakProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DecodeSystemPointer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DecodeSystemPointer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefineDosDeviceW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!DefineDosDeviceW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DelayLoadFailureHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DelayLoadFailureHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteBoundaryDescriptor" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DeleteBoundaryDescriptor h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteCriticalSection" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!DeleteCriticalSection 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteFiber" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DeleteFiber h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteFile2A" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!DeleteFile2A '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DeleteFile2W" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!DeleteFile2W '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DeleteFileTransactedA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!DeleteFileTransactedA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DeleteProcThreadAttributeList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DeleteProcThreadAttributeList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteSynchronizationBarrier" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!DeleteSynchronizationBarrier 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteTimerQueueEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!DeleteTimerQueueEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteTimerQueueTimer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!DeleteTimerQueueTimer 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteVolumeMountPointW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!DeleteVolumeMountPointW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DeviceIoControl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DeviceIoControl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DisableThreadLibraryCalls" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!DisableThreadLibraryCalls");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DisassociateCurrentThreadFromCallback" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DisassociateCurrentThreadFromCallback h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DiscardVirtualMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DiscardVirtualMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DnsHostnameToComputerNameExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!DnsHostnameToComputerNameExW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DuplicateConsoleHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DuplicateConsoleHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DuplicatePackageVirtualizationContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DuplicatePackageVirtualizationContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableProcessOptionalXStateFeatures" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EnableProcessOptionalXStateFeatures h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EncodeSystemPointer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EncodeSystemPointer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnterSynchronizationBarrier" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!EnterSynchronizationBarrier 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "EnumCalendarInfoExEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EnumCalendarInfoExEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumCalendarInfoExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumCalendarInfoExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumCalendarInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!EnumCalendarInfoW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDateFormatsExEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EnumDateFormatsExEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDateFormatsExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumDateFormatsExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDateFormatsW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumDateFormatsW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumLanguageGroupLocalesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!EnumLanguageGroupLocalesW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceLanguagesA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceLanguagesA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceLanguagesExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!EnumResourceLanguagesExA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceLanguagesExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!EnumResourceLanguagesExW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceLanguagesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceLanguagesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceNamesA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceNamesA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceNamesExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceNamesExA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceNamesExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceNamesExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceNamesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceNamesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceTypesA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceTypesA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceTypesExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceTypesExA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceTypesExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceTypesExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumResourceTypesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumResourceTypesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumSystemCodePagesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumSystemCodePagesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumSystemFirmwareTables" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EnumSystemFirmwareTables arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumSystemLanguageGroupsW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!EnumSystemLanguageGroupsW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumSystemLocalesA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!EnumSystemLocalesA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumSystemLocalesEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EnumSystemLocalesEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumSystemLocalesW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!EnumSystemLocalesW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumTimeFormatsEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EnumTimeFormatsEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumTimeFormatsW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumTimeFormatsW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumUILanguagesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!EnumUILanguagesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EscapeCommFunction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EscapeCommFunction arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExitVDM" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!ExitVDM arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExpungeConsoleCommandHistoryA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!ExpungeConsoleCommandHistoryA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ExpungeConsoleCommandHistoryW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!ExpungeConsoleCommandHistoryW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FatalAppExitA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!FatalAppExitA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FatalAppExitW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!FatalAppExitW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FillConsoleOutputAttribute" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FillConsoleOutputAttribute p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FillConsoleOutputCharacterA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!FillConsoleOutputCharacterA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FillConsoleOutputCharacterW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!FillConsoleOutputCharacterW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FindActCtxSectionGuid" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FindActCtxSectionGuid h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindActCtxSectionStringW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!FindActCtxSectionStringW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FindCloseChangeNotification" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!FindCloseChangeNotification");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindFirstChangeNotificationA" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstChangeNotificationA");
            emu.regs_mut().rax = helper::handler_create("FindFirstChangeNotificationA");
        }
        "FindFirstChangeNotificationW" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstChangeNotificationW");
            emu.regs_mut().rax = helper::handler_create("FindFirstChangeNotificationW");
        }
        "FindFirstFileExA" => {
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstFileExA");
            emu.regs_mut().rax = helper::handler_create("FindFirstFileExA");
        }
        "FindFirstFileExW" => {
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstFileExW");
            emu.regs_mut().rax = helper::handler_create("FindFirstFileExW");
        }
        "FindFirstFileNameTransactedW" => {
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstFileNameTransactedW");
            emu.regs_mut().rax = helper::handler_create("FindFirstFileNameTransactedW");
        }
        "FindFirstFileNameW" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstFileNameW");
            emu.regs_mut().rax = helper::handler_create("FindFirstFileNameW");
        }
        "FindFirstFileTransactedA" => {
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstFileTransactedA");
            emu.regs_mut().rax = helper::handler_create("FindFirstFileTransactedA");
        }
        "FindFirstFileTransactedW" => {
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstFileTransactedW");
            emu.regs_mut().rax = helper::handler_create("FindFirstFileTransactedW");
        }
        "FindFirstStreamTransactedW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!FindFirstStreamTransactedW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FindFirstStreamW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!FindFirstStreamW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FindFirstVolumeW" => {
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!FindFirstVolumeW");
            emu.regs_mut().rax = helper::handler_create("FindFirstVolumeW");
        }
        "FindNLSString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FindNLSString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindNLSStringEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FindNLSStringEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindNextChangeNotification" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!FindNextChangeNotification");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindNextFileNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!FindNextFileNameW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FindNextStreamW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!FindNextStreamW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "FindNextVolumeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!FindNextVolumeW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "FindPackagesByPackageFamily" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FindPackagesByPackageFamily arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindResourceExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            let hndl = helper::handler_create("resource");
            log_red!(
                emu,
                "kernel32!FindResourceExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x} => 0x{:x}",
                s,
                p1,
                p2,
                p3,
                hndl
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hndl;
        }
        "FindResourceExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            let hndl = helper::handler_create("resource");
            log_red!(
                emu,
                "kernel32!FindResourceExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x} => 0x{:x}",
                s,
                p1,
                p2,
                p3,
                hndl
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hndl;
        }
        "FindStringOrdinal" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FindStringOrdinal arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindVolumeClose" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!FindVolumeClose");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlsFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!FlsFree arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlsGetValue2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FlsGetValue2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlushConsoleInputBuffer" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FlushConsoleInputBuffer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlushFileBuffers" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!FlushFileBuffers");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlushInstructionCache" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FlushInstructionCache h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlushProcessWriteBuffers" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FlushProcessWriteBuffers h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlushViewOfFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FlushViewOfFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FoldStringW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!FoldStringW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "FormatApplicationUserModelId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FormatApplicationUserModelId arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FreeConsole" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!FreeConsole handle:0x{:x}", handle);
            if handle != 0 {
                helper::handler_close(handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FreeLibraryAndExitThread" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FreeLibraryAndExitThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FreeLibraryWhenCallbackReturns" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FreeLibraryWhenCallbackReturns h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FreeUserPhysicalPages" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!FreeUserPhysicalPages h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GenerateConsoleCtrlEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!GenerateConsoleCtrlEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetAppContainerAce" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetAppContainerAce h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAppContainerNamedObjectPath" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetAppContainerNamedObjectPath h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetApplicationRecoveryCallback" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetApplicationRecoveryCallback h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetApplicationRecoveryCallbackWorker" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetApplicationRecoveryCallbackWorker h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetApplicationRestartSettings" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetApplicationRestartSettings h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetApplicationUserModelId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetApplicationUserModelId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetBinaryType" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetBinaryType h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetBinaryTypeA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!GetBinaryTypeA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetCPInfoExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!GetCPInfoExA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetCPInfoExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!GetCPInfoExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetCachedSigningLevel" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCachedSigningLevel h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCalendarInfoEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCalendarInfoEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCalendarInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetCalendarInfoW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "GetCommConfig" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCommConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCommMask" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCommMask h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCommModemStatus" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCommModemStatus h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCommProperties" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCommProperties h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCommState" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCommState h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCommTimeouts" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCommTimeouts h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCompressedFileSizeA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!GetCompressedFileSizeA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 4096;
        }
        "GetCompressedFileSizeTransactedA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetCompressedFileSizeTransactedA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetCompressedFileSizeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!GetCompressedFileSizeW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 4096;
        }
        "GetComputerNameExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetComputerNameExA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetComputerNameExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetComputerNameExW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!GetConsoleAliasA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasExesA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!GetConsoleAliasExesA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasExesLengthA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleAliasExesLengthA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasExesLengthW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleAliasExesLengthW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasExesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!GetConsoleAliasExesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!GetConsoleAliasW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasesA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!GetConsoleAliasesA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasesLengthA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleAliasesLengthA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasesLengthW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleAliasesLengthW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleAliasesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!GetConsoleAliasesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleCommandHistoryA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleCommandHistoryA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleCommandHistoryLengthA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleCommandHistoryLengthA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleCommandHistoryLengthW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleCommandHistoryLengthW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleCommandHistoryW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleCommandHistoryW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleCursorInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleCursorInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleCursorMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleCursorMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleDisplayMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleDisplayMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleFontSize" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleFontSize h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleHistoryInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleHistoryInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleInputExeNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleInputExeNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleInputExeNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleInputExeNameW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleNlsMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleNlsMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleOriginalTitleA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleOriginalTitleA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleOriginalTitleW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetConsoleOriginalTitleW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetConsoleProcessList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleProcessList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleScreenBufferInfo" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetConsoleScreenBufferInfo p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleScreenBufferInfoEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleScreenBufferInfoEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleSelectionInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleSelectionInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetConsoleTitleA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!GetConsoleTitleA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "GetConsoleTitleW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!GetConsoleTitleW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "GetConsoleWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetConsoleWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrencyFormatEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrencyFormatEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentActCtx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentActCtx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentApplicationUserModelId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentApplicationUserModelId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentConsoleFont" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentConsoleFont h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentConsoleFontEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentConsoleFontEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentPackageFamilyName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentPackageFamilyName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentPackageFullName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentPackageFullName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentPackageId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentPackageId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentPackageInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentPackageInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentPackagePath" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentPackagePath h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentPackageVirtualizationContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentPackageVirtualizationContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentProcessorNumber" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentProcessorNumber h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentProcessorNumberEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentProcessorNumberEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentThreadStackLimits" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentThreadStackLimits h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDateFormatA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetDateFormatA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "GetDateFormatEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetDateFormatEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDateFormatW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetDateFormatW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "GetDiskFreeSpaceW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetDiskFreeSpaceW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetDiskSpaceInformationA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetDiskSpaceInformationA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetDiskSpaceInformationW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetDiskSpaceInformationW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetDurationFormatEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetDurationFormatEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDynamicTimeZoneInformation" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetDynamicTimeZoneInformation");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetEnabledXStateFeatures" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetEnabledXStateFeatures h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetEnvironmentStringsA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!GetEnvironmentStringsA arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetEnvironmentVariableA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetEnvironmentVariableA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "GetEraNameCountedString" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetEraNameCountedString h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetErrorMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetErrorMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetExitCodeProcess" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetExitCodeProcess p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetExitCodeThread" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetExitCodeThread p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetFileAttributesExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!GetFileAttributesExA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetFileAttributesExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!GetFileAttributesExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetFileAttributesTransactedW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetFileAttributesTransactedW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetFileInformationByHandle" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetFileInformationByHandle p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetFileInformationByHandleEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetFileInformationByHandleEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetFileInformationByName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetFileInformationByName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetFileMUIInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetFileMUIInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetFileMUIPath" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetFileMUIPath h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetFileTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetFileTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetFinalPathNameByHandleA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetFinalPathNameByHandleA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "GetFinalPathNameByHandleW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetFinalPathNameByHandleW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "GetFirmwareEnvironmentVariableExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetFirmwareEnvironmentVariableExA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetFirmwareEnvironmentVariableExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetFirmwareEnvironmentVariableExW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetHandleInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetHandleInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetIoRingInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetIoRingInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLargePageMinimum" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetLargePageMinimum h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLargestConsoleWindowSize" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetLargestConsoleWindowSize h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLocaleInfoEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetLocaleInfoEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLogicalDriveStringsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetLogicalDriveStringsA '{}' p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 4;
        }
        "GetLogicalDriveStringsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetLogicalDriveStringsW '{}' p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 4;
        }
        "GetLogicalProcessorInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetLogicalProcessorInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLogicalProcessorInformationEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetLogicalProcessorInformationEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLongPathNameTransactedA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetLongPathNameTransactedA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetLongPathNameTransactedW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetLongPathNameTransactedW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMachineTypeAttributes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetMachineTypeAttributes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMemoryErrorHandlingCapabilities" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetMemoryErrorHandlingCapabilities h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetModuleHandleExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetModuleHandleExA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetNLSVersion" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNLSVersion h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNLSVersionEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNLSVersionEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNamedPipeAttribute" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNamedPipeAttribute h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNamedPipeClientComputerNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetNamedPipeClientComputerNameW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetNamedPipeHandleStateW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetNamedPipeHandleStateW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetNamedPipeInfo" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNamedPipeInfo p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNamedPipeServerProcessId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNamedPipeServerProcessId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNextVDMCommand" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNextVDMCommand h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNumaHighestNodeNumber" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNumaHighestNodeNumber h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNumaNodeProcessorMask2" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNumaNodeProcessorMask2 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNumaNodeProcessorMaskEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNumaNodeProcessorMaskEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNumaProximityNodeEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNumaProximityNodeEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNumberFormatEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNumberFormatEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNumberOfConsoleInputEvents" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!GetNumberOfConsoleInputEvents 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetNumberOfConsoleMouseButtons" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNumberOfConsoleMouseButtons h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetOverlappedResult" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetOverlappedResult h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetOverlappedResultEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetOverlappedResultEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPackageApplicationIds" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPackageApplicationIds h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPackageFamilyName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPackageFamilyName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPackageFullName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPackageFullName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPackageId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPackageId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPackageInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPackageInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPackagePath" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPackagePath h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPackagePathByFullName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPackagePathByFullName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPackagesByPackageFamily" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPackagesByPackageFamily h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPhysicallyInstalledSystemMemory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetPhysicallyInstalledSystemMemory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPriorityClass" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetPriorityClass");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x20;
        }
        "GetProcessDefaultCpuSetMasks" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessDefaultCpuSetMasks h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessDefaultCpuSets" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessDefaultCpuSets h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessGroupAffinity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessGroupAffinity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessHandleCount" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetProcessHandleCount p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessHeaps" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessHeaps h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessId" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetProcessId");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x1234;
        }
        "GetProcessIdOfThread" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessIdOfThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessMitigationPolicy" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessMitigationPolicy h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessPreferredUILanguages" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessPreferredUILanguages h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessPriorityBoost" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessPriorityBoost h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessShutdownParameters" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessShutdownParameters h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessTimes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessTimes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessVersion" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessVersion h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessWorkingSetSizeEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessWorkingSetSizeEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessesInVirtualizationContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessesInVirtualizationContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessorSystemCycleTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProcessorSystemCycleTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProductInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetProductInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetQueuedCompletionStatus" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetQueuedCompletionStatus h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetQueuedCompletionStatusEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetQueuedCompletionStatusEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetStagedPackagePathByFullName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetStagedPackagePathByFullName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetStateFolder" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetStateFolder h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetStringScripts" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetStringScripts h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetStringTypeA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetStringTypeA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetStringTypeExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!GetStringTypeExA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetStringTypeExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!GetStringTypeExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetSystemAppDataKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemAppDataKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemCpuSetInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemCpuSetInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemDefaultLCID" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemDefaultLCID h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemDefaultLangID" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemDefaultLangID h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemDefaultLocaleName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemDefaultLocaleName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemDefaultUILanguage" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemDefaultUILanguage h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemFileCacheSize" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemFileCacheSize h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemFirmwareTable" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemFirmwareTable h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemPreferredUILanguages" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemPreferredUILanguages h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemTimeAdjustment" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemTimeAdjustment h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemTimePreciseAsFileTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemTimePreciseAsFileTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemTimes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetSystemTimes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemWow64DirectoryA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetSystemWow64DirectoryA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetSystemWow64DirectoryW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetSystemWow64DirectoryW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTempFileNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetTempFileNameA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTempFileNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetTempFileNameW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTempPath2A" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!GetTempPath2A '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTempPath2W" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!GetTempPath2W '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetThreadDescription" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadDescription h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadEnabledXStateFeatures" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadEnabledXStateFeatures h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadErrorMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadErrorMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadGroupAffinity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadGroupAffinity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadIOPendingFlag" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadIOPendingFlag h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadIdealProcessorEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadIdealProcessorEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadPriority" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!GetThreadPriority");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 2;
        }
        "GetThreadPriorityBoost" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadPriorityBoost h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadSelectedCpuSetMasks" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadSelectedCpuSetMasks h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadSelectedCpuSets" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadSelectedCpuSets h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadTimes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetThreadTimes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTimeFormatA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetTimeFormatA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "GetTimeFormatAWorker" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetTimeFormatAWorker h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTimeFormatEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetTimeFormatEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTimeFormatW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetTimeFormatW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "GetTimeZoneInformationForYear" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetTimeZoneInformationForYear h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUILanguageInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetUILanguageInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUserDefaultLocaleName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetUserDefaultLocaleName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUserPreferredUILanguages" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetUserPreferredUILanguages h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetVDMCurrentDirectories" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetVDMCurrentDirectories h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetVolumeInformationByHandleW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetVolumeInformationByHandleW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetVolumeNameForVolumeMountPointW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetVolumeNameForVolumeMountPointW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetVolumePathNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetVolumePathNameA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetVolumePathNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!GetVolumePathNameW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetVolumePathNamesForVolumeNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!GetVolumePathNamesForVolumeNameW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetWriteWatch" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetWriteWatch h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetXStateFeaturesMask" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetXStateFeaturesMask h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GlobalCompact" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GlobalCompact arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GlobalMemoryStatusEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GlobalMemoryStatusEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HeapCompact" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!HeapCompact p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 4096;
        }
        "HeapLock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!HeapLock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "HeapQueryInformation" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!HeapQueryInformation p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HeapSummary" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!HeapSummary arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HeapUnlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!HeapUnlock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "HeapValidate" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!HeapValidate p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HeapWalk" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!HeapWalk arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IdnToAscii" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IdnToAscii arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IdnToNameprepUnicode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IdnToNameprepUnicode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IdnToUnicode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IdnToUnicode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitOnceBeginInitialize" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!InitOnceBeginInitialize p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitOnceComplete" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!InitOnceComplete p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitOnceExecuteOnce" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!InitOnceExecuteOnce p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitOnceInitialize" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!InitOnceInitialize");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeConditionVariable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!InitializeConditionVariable 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InitializeContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!InitializeContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeContext2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!InitializeContext2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeEnclave" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!InitializeEnclave arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeProcThreadAttributeList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!InitializeProcThreadAttributeList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeSListHead" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!InitializeSListHead arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeSynchronizationBarrier" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "kernel32!InitializeSynchronizationBarrier 0x{:x}",
                arg1
            );
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InstallELAMCertificateInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!InstallELAMCertificateInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InterlockedCompareExchange64" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!InterlockedCompareExchange64 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InterlockedFlushSList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!InterlockedFlushSList 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InterlockedPopEntrySList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!InterlockedPopEntrySList 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InterlockedPushEntrySList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!InterlockedPushEntrySList 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InterlockedPushListSList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!InterlockedPushListSList 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "InterlockedPushListSListEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!InterlockedPushListSListEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "IsDBCSLeadByte" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsDBCSLeadByte arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsDBCSLeadByteEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsDBCSLeadByteEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsEnclaveTypeSupported" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsEnclaveTypeSupported arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsIoRingOpSupported" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsIoRingOpSupported arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsNLSDefinedString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsNLSDefinedString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsNativeVhdBoot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsNativeVhdBoot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsNormalizedString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsNormalizedString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsProcessCritical" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!IsProcessCritical 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "IsProcessInJob" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsProcessInJob h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsThreadpoolTimerSet" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!IsThreadpoolTimerSet 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "IsUserCetAvailableInEnvironment" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsUserCetAvailableInEnvironment arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsValidLanguageGroup" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!IsValidLanguageGroup p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsValidLocaleName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsValidLocaleName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsValidNLSVersion" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsValidNLSVersion arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWow64GuestMachineSupported" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsWow64GuestMachineSupported arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWow64Process2" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!IsWow64Process2 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32EmptyWorkingSet" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32EmptyWorkingSet arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32EnumDeviceDrivers" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32EnumDeviceDrivers arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32EnumPageFilesA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!K32EnumPageFilesA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32EnumPageFilesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!K32EnumPageFilesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32EnumProcessModules" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32EnumProcessModules h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32EnumProcessModulesEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32EnumProcessModulesEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32EnumProcesses" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32EnumProcesses h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32GetDeviceDriverBaseNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!K32GetDeviceDriverBaseNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetDeviceDriverBaseNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!K32GetDeviceDriverBaseNameW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetDeviceDriverFileNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!K32GetDeviceDriverFileNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetDeviceDriverFileNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!K32GetDeviceDriverFileNameW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetMappedFileNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!K32GetMappedFileNameA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetMappedFileNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!K32GetMappedFileNameW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetModuleBaseNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!K32GetModuleBaseNameA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetModuleBaseNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!K32GetModuleBaseNameW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetModuleFileNameExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!K32GetModuleFileNameExA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetModuleFileNameExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!K32GetModuleFileNameExW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetModuleInformation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32GetModuleInformation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32GetPerformanceInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32GetPerformanceInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32GetProcessImageFileNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!K32GetProcessImageFileNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetProcessImageFileNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!K32GetProcessImageFileNameW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "K32GetProcessMemoryInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32GetProcessMemoryInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32GetWsChanges" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32GetWsChanges arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32GetWsChangesEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32GetWsChangesEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32InitializeProcessForWsWatch" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32InitializeProcessForWsWatch h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32QueryWorkingSet" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32QueryWorkingSet arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "K32QueryWorkingSetEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!K32QueryWorkingSetEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LCIDToLocaleName" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LCIDToLocaleName p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LCMapStringA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!LCMapStringA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "LCMapStringEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LCMapStringEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LZCloseFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LZCloseFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LZCreateFileW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!LZCreateFileW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LZDone" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!LZDone arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LeaveCriticalSectionWhenCallbackReturns" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "kernel32!LeaveCriticalSectionWhenCallbackReturns 0x{:x}",
                arg1
            );
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LoadAppInitDlls" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LoadAppInitDlls arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadEnclaveData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LoadEnclaveData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadModule" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LoadModule arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadPackagedLibrary" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LoadPackagedLibrary arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadStringBaseExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!LoadStringBaseExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LocalFileTimeToFileTime" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!LocalFileTimeToFileTime p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LocalFileTimeToLocalSystemTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LocalFileTimeToLocalSystemTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LocalLock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!LocalLock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LocalReAlloc" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!LocalReAlloc p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x100000;
        }
        "LocalSystemTimeToLocalFileTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LocalSystemTimeToLocalFileTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LocalUnlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!LocalUnlock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LocaleNameToLCID" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!LocaleNameToLCID p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x0409;
        }
        "LocateXStateFeature" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LocateXStateFeature arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LockFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!LockFile 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LockFileEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!LockFileEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LogUnexpectedCodepath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!LogUnexpectedCodepath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapUserPhysicalPages" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!MapUserPhysicalPages arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapUserPhysicalPagesScatter" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!MapUserPhysicalPagesScatter arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapViewOfFileEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!MapViewOfFileEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x100000;
        }
        "MapViewOfFileExNuma" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!MapViewOfFileExNuma h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapViewOfFileFromApp" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!MapViewOfFileFromApp h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MoveFileExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!MoveFileExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "MoveFileWithProgressA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!MoveFileWithProgressA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "MoveFileWithProgressW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!MoveFileWithProgressW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "NeedCurrentDirectoryForExePathA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!NeedCurrentDirectoryForExePathA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "NeedCurrentDirectoryForExePathW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!NeedCurrentDirectoryForExePathW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "NlsCheckPolicy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!NlsCheckPolicy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NlsGetCacheUpdateCount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!NlsGetCacheUpdateCount arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NlsUpdateLocale" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!NlsUpdateLocale arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NlsUpdateSystemLocale" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!NlsUpdateSystemLocale arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NormalizeString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!NormalizeString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NotifyMountMgr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!NotifyMountMgr 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "OfferVirtualMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!OfferVirtualMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenConsoleW" => {
            log_red!(emu, "kernel32!OpenConsoleW");
            emu.regs_mut().rax = helper::handler_create("OpenConsoleW");
        }
        "OpenConsoleWStub" => {
            log_red!(emu, "kernel32!OpenConsoleWStub");
            emu.regs_mut().rax = helper::handler_create("OpenConsoleWStub");
        }
        "OpenEventA" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenEventA");
            emu.regs_mut().rax = helper::handler_create("OpenEventA");
        }
        "OpenEventW" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenEventW");
            emu.regs_mut().rax = helper::handler_create("OpenEventW");
        }
        "OpenFileById" => {
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenFileById");
            emu.regs_mut().rax = helper::handler_create("OpenFileById");
        }
        "OpenFileMappingW" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenFileMappingW");
            emu.regs_mut().rax = helper::handler_create("OpenFileMappingW");
        }
        "OpenMutexW" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenMutexW");
            emu.regs_mut().rax = helper::handler_create("OpenMutexW");
        }
        "OpenPackageInfoByFullName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!OpenPackageInfoByFullName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenPrivateNamespaceW" => {
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenPrivateNamespaceW");
            emu.regs_mut().rax = helper::handler_create("OpenPrivateNamespaceW");
        }
        "OpenProfileUserMapping" => {
            log_red!(emu, "kernel32!OpenProfileUserMapping");
            emu.regs_mut().rax = helper::handler_create("OpenProfileUserMapping");
        }
        "OpenSemaphoreA" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenSemaphoreA");
            emu.regs_mut().rax = helper::handler_create("OpenSemaphoreA");
        }
        "OpenSemaphoreW" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenSemaphoreW");
            emu.regs_mut().rax = helper::handler_create("OpenSemaphoreW");
        }
        "OpenState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!OpenState arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenStateExplicit" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!OpenStateExplicit arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenThreadToken" => {
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenThreadToken");
            emu.regs_mut().rax = helper::handler_create("OpenThreadToken");
        }
        "OpenWaitableTimerA" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenWaitableTimerA");
            emu.regs_mut().rax = helper::handler_create("OpenWaitableTimerA");
        }
        "OpenWaitableTimerW" => {
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            log_red!(emu, "kernel32!OpenWaitableTimerW");
            emu.regs_mut().rax = helper::handler_create("OpenWaitableTimerW");
        }
        "PackageFamilyNameFromFullName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PackageFamilyNameFromFullName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PackageFamilyNameFromId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PackageFamilyNameFromId arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PackageFullNameFromId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PackageFullNameFromId arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PackageIdFromFullName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PackageIdFromFullName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PackageNameAndPublisherIdFromFamilyName" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PackageNameAndPublisherIdFromFamilyName arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ParseApplicationUserModelId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ParseApplicationUserModelId arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PeekConsoleInputA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!PeekConsoleInputA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PeekConsoleInputW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!PeekConsoleInputW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PeekNamedPipe" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PeekNamedPipe p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PopIoRingCompletion" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PopIoRingCompletion arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PostQueuedCompletionStatus" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PostQueuedCompletionStatus arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrefetchVirtualMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PrefetchVirtualMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivCopyFileExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!PrivCopyFileExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ProcessIdToSessionId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ProcessIdToSessionId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssCaptureSnapshot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssCaptureSnapshot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssDuplicateSnapshot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssDuplicateSnapshot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssFreeSnapshot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssFreeSnapshot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssQuerySnapshot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssQuerySnapshot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkMarkerCreate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkMarkerCreate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkMarkerFree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkMarkerFree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkMarkerGetPosition" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkMarkerGetPosition arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkMarkerRewind" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkMarkerRewind arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkMarkerSeek" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkMarkerSeek arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkMarkerSeekToBeginning" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkMarkerSeekToBeginning arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkMarkerSetPosition" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkMarkerSetPosition arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkMarkerTell" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkMarkerTell arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PssWalkSnapshot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PssWalkSnapshot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PulseEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!PulseEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "PurgeComm" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!PurgeComm arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryActCtxSettingsW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!QueryActCtxSettingsW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryActCtxW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!QueryActCtxW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryDepthSList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryDepthSList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryDosDeviceW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!QueryDosDeviceW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryFullProcessImageNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!QueryFullProcessImageNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryFullProcessImageNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!QueryFullProcessImageNameW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryIdleProcessorCycleTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryIdleProcessorCycleTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryIdleProcessorCycleTimeEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryIdleProcessorCycleTimeEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryIoRingCapabilities" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryIoRingCapabilities h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryMemoryResourceNotification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryMemoryResourceNotification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryProcessAffinityUpdateMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryProcessAffinityUpdateMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryProcessCycleTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryProcessCycleTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryProtectedPolicy" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryProtectedPolicy h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryThreadCycleTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryThreadCycleTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryThreadpoolStackInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryThreadpoolStackInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryUnbiasedInterruptTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryUnbiasedInterruptTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueueUserAPC" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueueUserAPC arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueueUserAPC2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueueUserAPC2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueueUserWorkItem" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueueUserWorkItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QuirkGetData2Worker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QuirkGetData2Worker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QuirkIsEnabled3Worker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QuirkIsEnabled3Worker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RaiseFailFastException" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RaiseFailFastException arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReOpenFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReOpenFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!ReadConsoleA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleInputA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!ReadConsoleInputA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleInputExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!ReadConsoleInputExA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleInputExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!ReadConsoleInputExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleInputW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!ReadConsoleInputW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleOutputA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!ReadConsoleOutputA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleOutputAttribute" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReadConsoleOutputAttribute h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleOutputCharacterA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!ReadConsoleOutputCharacterA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleOutputCharacterW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!ReadConsoleOutputCharacterW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleOutputW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!ReadConsoleOutputW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadConsoleW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!ReadConsoleW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadDirectoryChangesExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!ReadDirectoryChangesExW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadDirectoryChangesW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!ReadDirectoryChangesW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReadFileEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReadFileEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReadFileScatter" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReadFileScatter h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReclaimVirtualMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReclaimVirtualMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegCopyTreeW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!RegCopyTreeW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegDeleteKeyExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegDeleteKeyExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteKeyExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegDeleteKeyExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteTreeA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!RegDeleteTreeA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegDeleteTreeW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!RegDeleteTreeW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegDeleteValueA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!RegDeleteValueA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDeleteValueW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!RegDeleteValueW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegDisablePredefinedCacheEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegDisablePredefinedCacheEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegEnumKeyExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegEnumKeyExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 259;
        }
        "RegEnumKeyExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegEnumKeyExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 259;
        }
        "RegEnumValueA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegEnumValueA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 259;
        }
        "RegEnumValueW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegEnumValueW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 259;
        }
        "RegFlushKey" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!RegFlushKey");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegGetKeySecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegGetKeySecurity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegGetValueA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!RegGetValueA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegGetValueW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!RegGetValueW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegLoadKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegLoadKeyA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegLoadKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegLoadKeyW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegLoadMUIStringA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!RegLoadMUIStringA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegLoadMUIStringW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!RegLoadMUIStringW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegNotifyChangeKeyValue" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegNotifyChangeKeyValue p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegOpenCurrentUser" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegOpenCurrentUser arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegOpenKeyExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegOpenKeyExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "RegOpenUserClassesRoot" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegOpenUserClassesRoot arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegQueryInfoKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegQueryInfoKeyA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..12 {
                emu.stack_pop32(false);
            }
        }
        "RegQueryInfoKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegQueryInfoKeyW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..12 {
                emu.stack_pop32(false);
            }
        }
        "RegQueryValueExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegQueryValueExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 2;
        }
        "RegQueryValueExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegQueryValueExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 2;
        }
        "RegRestoreKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegRestoreKeyA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegRestoreKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!RegRestoreKeyW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "RegSaveKeyExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!RegSaveKeyExA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegSaveKeyExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!RegSaveKeyExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegSetKeySecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegSetKeySecurity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegUnLoadKeyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!RegUnLoadKeyA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegUnLoadKeyW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!RegUnLoadKeyW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "RegisterBadMemoryNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegisterBadMemoryNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterWaitForSingleObject" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!RegisterWaitForSingleObject 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RegisterWaitForSingleObjectEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!RegisterWaitForSingleObjectEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RegisterWowBaseHandlers" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegisterWowBaseHandlers h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterWowExec" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RegisterWowExec arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReleaseActCtx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReleaseActCtx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReleaseActCtxWorker" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReleaseActCtxWorker h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReleaseMutex" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!ReleaseMutex 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ReleaseMutexWhenCallbackReturns" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!ReleaseMutexWhenCallbackReturns 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ReleasePackageVirtualizationContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReleasePackageVirtualizationContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReleasePseudoConsole" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReleasePseudoConsole h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReleaseSemaphore" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!ReleaseSemaphore 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ReleaseSemaphoreWhenCallbackReturns" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "kernel32!ReleaseSemaphoreWhenCallbackReturns 0x{:x}",
                arg1
            );
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RemoveDirectory2A" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!RemoveDirectory2A '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RemoveDirectory2W" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!RemoveDirectory2W '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RemoveDirectoryTransactedA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!RemoveDirectoryTransactedA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RemoveDllDirectory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RemoveDllDirectory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemoveLocalAlternateComputerNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!RemoveLocalAlternateComputerNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RemoveVectoredContinueHandler" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RemoveVectoredContinueHandler h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemoveVectoredExceptionHandler" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RemoveVectoredExceptionHandler h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReplaceFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ReplaceFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReplaceFileW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!ReplaceFileW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RequestDeviceWakeup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RequestDeviceWakeup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ResetWriteWatch" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ResetWriteWatch h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ResizePseudoConsole" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ResizePseudoConsole arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ResolveDelayLoadedAPI" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ResolveDelayLoadedAPI arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ResolveDelayLoadsFromDll" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ResolveDelayLoadsFromDll arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ResolveLocaleName" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ResolveLocaleName p1:0x{:x} p2:0x{:x}",
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RestoreLastError" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RestoreLastError arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlCaptureContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlCaptureContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlCaptureStackBackTrace" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlCaptureStackBackTrace arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlFillMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlFillMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlMoveMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlMoveMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlPcToFileHeader" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlPcToFileHeader h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlUnwind" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlUnwind arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlZeroMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlZeroMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollConsoleScreenBufferA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!ScrollConsoleScreenBufferA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ScrollConsoleScreenBufferW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!ScrollConsoleScreenBufferW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SearchPathA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!SearchPathA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "SearchPathW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!SearchPathW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "SetCachedSigningLevel" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetCachedSigningLevel h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCalendarInfoW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!SetCalendarInfoW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetCommBreak" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetCommBreak h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCommConfig" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetCommConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCommMask" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetCommMask h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCommState" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetCommState h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCommTimeouts" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetCommTimeouts h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetComputerNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!SetComputerNameA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetComputerNameEx2W" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!SetComputerNameEx2W '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetComputerNameExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!SetComputerNameExA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetComputerNameExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!SetComputerNameExW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetComputerNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!SetComputerNameW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetConsoleActiveScreenBuffer" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleActiveScreenBuffer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleCP" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleCP h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleCtrlHandler" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetConsoleCtrlHandler p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleCursorInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleCursorInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleCursorMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleCursorMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleCursorPosition" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetConsoleCursorPosition p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleDisplayMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleDisplayMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleHistoryInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleHistoryInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleInputExeNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!SetConsoleInputExeNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetConsoleInputExeNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!SetConsoleInputExeNameW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetConsoleMode" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetConsoleMode p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleNlsMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleNlsMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleNumberOfCommandsA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!SetConsoleNumberOfCommandsA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetConsoleNumberOfCommandsW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!SetConsoleNumberOfCommandsW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetConsoleOutputCP" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleOutputCP h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleScreenBufferInfoEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleScreenBufferInfoEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleScreenBufferSize" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleScreenBufferSize h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleTextAttribute" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetConsoleTextAttribute p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetConsoleTitleA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!SetConsoleTitleA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetConsoleTitleW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!SetConsoleTitleW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetConsoleWindowInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetConsoleWindowInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCriticalSectionSpinCount" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetCriticalSectionSpinCount 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetCurrentConsoleFontEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetCurrentConsoleFontEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCurrentDirectoryW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!SetCurrentDirectoryW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetDefaultCommConfigW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!SetDefaultCommConfigW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetDefaultDllDirectories" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetDefaultDllDirectories h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDynamicTimeZoneInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetDynamicTimeZoneInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetEndOfFile" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetEndOfFile");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetEnvironmentStringsW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!SetEnvironmentStringsW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetEventWhenCallbackReturns" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetEventWhenCallbackReturns 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetFileApisToANSI" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetFileApisToANSI h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFileApisToOEM" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetFileApisToOEM h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFileAttributesTransactedA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!SetFileAttributesTransactedA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetFileCompletionNotificationModes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetFileCompletionNotificationModes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFileInformationByHandle" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetFileInformationByHandle p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFileIoOverlappedRange" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetFileIoOverlappedRange h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFilePointerEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetFilePointerEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFileTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetFileTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFileValidData" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetFileValidData p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFirmwareEnvironmentVariableExA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!SetFirmwareEnvironmentVariableExA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetFirmwareEnvironmentVariableExW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!SetFirmwareEnvironmentVariableExW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetHandleContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetHandleContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetHandleInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetHandleInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetIoRingCompletionEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetIoRingCompletionEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetLastConsoleEventActive" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetLastConsoleEventActive 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetLocalPrimaryComputerNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!SetLocalPrimaryComputerNameA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetLocalTime" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetLocalTime");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetLocaleInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!SetLocaleInfoW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetNamedPipeHandleState" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetNamedPipeHandleState p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetPriorityClass" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetPriorityClass p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessAffinityUpdateMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessAffinityUpdateMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDefaultCpuSetMasks" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessDefaultCpuSetMasks h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDefaultCpuSets" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessDefaultCpuSets h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDynamicEHContinuationTargets" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessDynamicEHContinuationTargets h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDynamicEnforcedCetCompatibleRanges" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessDynamicEnforcedCetCompatibleRanges h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessMitigationPolicy" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessMitigationPolicy h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessPreferredUILanguages" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessPreferredUILanguages h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessPriorityBoost" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessPriorityBoost h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessShutdownParameters" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessShutdownParameters h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessWorkingSetSizeEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProcessWorkingSetSizeEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProtectedPolicy" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetProtectedPolicy h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSearchPathMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetSearchPathMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetStdHandle" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetStdHandle p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetStdHandleEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetStdHandleEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSystemFileCacheSize" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetSystemFileCacheSize h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSystemTime" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetSystemTime");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetTermsrvAppInstallMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetTermsrvAppInstallMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadDescription" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadDescription h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadErrorMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadErrorMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadGroupAffinity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadGroupAffinity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadIdealProcessor" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadIdealProcessor h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadIdealProcessorEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadIdealProcessorEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadPreferredUILanguages" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadPreferredUILanguages h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadPriority" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetThreadPriority p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadPriorityBoost" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadPriorityBoost h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadSelectedCpuSetMasks" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadSelectedCpuSetMasks h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadSelectedCpuSets" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadSelectedCpuSets h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadStackGuarantee" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadStackGuarantee h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadToken" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadToken h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadUILanguage" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadUILanguage h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadpoolStackInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadpoolStackInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadpoolThreadMaximum" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadpoolThreadMaximum h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadpoolThreadMinimum" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetThreadpoolThreadMinimum h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadpoolTimer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetThreadpoolTimer 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetThreadpoolTimerEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetThreadpoolTimerEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetThreadpoolWait" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetThreadpoolWait 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetThreadpoolWaitEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetThreadpoolWaitEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetTimeZoneInformation" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetTimeZoneInformation");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetVDMCurrentDirectories" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetVDMCurrentDirectories h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetVolumeMountPointWStub" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetVolumeMountPointWStub h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWaitableTimer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetWaitableTimer 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetWaitableTimerEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SetWaitableTimerEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetXStateFeaturesMask" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetXStateFeaturesMask h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetupComm" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SetupComm h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SignalObjectAndWait" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SignalObjectAndWait 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SleepConditionVariableCS" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SleepConditionVariableCS 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SleepConditionVariableSRW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SleepConditionVariableSRW 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SleepEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SleepEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SortCloseHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SortCloseHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SortGetHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SortGetHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "StartThreadpoolIo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!StartThreadpoolIo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SubmitIoRing" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SubmitIoRing arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SubmitThreadpoolWork" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SubmitThreadpoolWork h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SuspendThread" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!SuspendThread");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwitchToFiber" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SwitchToFiber arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwitchToThread" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!SwitchToThread 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SystemTimeToTzSpecificLocalTimeEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SystemTimeToTzSpecificLocalTimeEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TerminateJobObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TerminateJobObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TerminateThread" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!TerminateThread p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TermsrvAppInstallMode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TermsrvAppInstallMode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TermsrvGetWindowsDirectoryA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!TermsrvGetWindowsDirectoryA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "TermsrvGetWindowsDirectoryW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!TermsrvGetWindowsDirectoryW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "TermsrvSetKeySecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TermsrvSetKeySecurity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TlsGetValue2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TlsGetValue2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Toolhelp32ReadProcessMemory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!Toolhelp32ReadProcessMemory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TransactNamedPipe" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TransactNamedPipe p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TransmitCommChar" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TransmitCommChar arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TryEnterCriticalSection" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!TryEnterCriticalSection 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "TrySubmitThreadpoolCallback" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TrySubmitThreadpoolCallback h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TzSpecificLocalTimeToSystemTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TzSpecificLocalTimeToSystemTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TzSpecificLocalTimeToSystemTimeEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!TzSpecificLocalTimeToSystemTimeEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UTRegister" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!UTRegister arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UTUnRegister" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!UTUnRegister arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnlockFile" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!UnlockFile 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UnlockFileEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!UnlockFileEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UnmapViewOfFile" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!UnmapViewOfFile");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnmapViewOfFileEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!UnmapViewOfFileEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterBadMemoryNotification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!UnregisterBadMemoryNotification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterConsoleIME" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!UnregisterConsoleIME h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterWaitEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!UnregisterWaitEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UpdateProcThreadAttribute" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!UpdateProcThreadAttribute h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VerLanguageNameA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!VerLanguageNameA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "VerLanguageNameW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!VerLanguageNameW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "VerSetConditionMask" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!VerSetConditionMask arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VerifyScripts" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!VerifyScripts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VirtualFreeEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!VirtualFreeEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VirtualUnlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!VirtualUnlock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WTSGetActiveConsoleSessionId" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WTSGetActiveConsoleSessionId arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WaitCommEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitCommEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForDebugEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitForDebugEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForDebugEventEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitForDebugEventEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForMultipleObjectsEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitForMultipleObjectsEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForSingleObjectEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitForSingleObjectEx 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForThreadpoolIoCallbacks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitForThreadpoolIoCallbacks 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForThreadpoolTimerCallbacks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitForThreadpoolTimerCallbacks 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForThreadpoolWaitCallbacks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitForThreadpoolWaitCallbacks 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForThreadpoolWorkCallbacks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitForThreadpoolWorkCallbacks 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitNamedPipeW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WaitNamedPipeW 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WakeAllConditionVariable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WakeAllConditionVariable 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WakeConditionVariable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WakeConditionVariable 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WerGetFlags" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerGetFlags arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerRegisterAdditionalProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerRegisterAdditionalProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerRegisterAppLocalDump" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerRegisterAppLocalDump arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerRegisterCustomMetadata" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerRegisterCustomMetadata arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerRegisterExcludedMemoryBlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WerRegisterExcludedMemoryBlock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WerRegisterFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerRegisterFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerRegisterMemoryBlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WerRegisterMemoryBlock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WerRegisterMemoryBlockWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WerRegisterMemoryBlockWorker 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WerRegisterRuntimeExceptionModule" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerRegisterRuntimeExceptionModule arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerRegisterRuntimeExceptionModuleWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerRegisterRuntimeExceptionModuleWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerSetFlagsWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerSetFlagsWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerUnregisterAdditionalProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerUnregisterAdditionalProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerUnregisterAppLocalDump" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerUnregisterAppLocalDump arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerUnregisterCustomMetadata" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerUnregisterCustomMetadata arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerUnregisterExcludedMemoryBlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(
                emu,
                "kernel32!WerUnregisterExcludedMemoryBlock 0x{:x}",
                arg1
            );
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WerUnregisterFile" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerUnregisterFile h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerUnregisterMemoryBlock" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!WerUnregisterMemoryBlock 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WerUnregisterRuntimeExceptionModule" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerUnregisterRuntimeExceptionModule arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerUnregisterRuntimeExceptionModuleWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerUnregisterRuntimeExceptionModuleWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WerpNotifyLoadStringResourceWorker" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WerpNotifyLoadStringResourceWorker arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Wow64DisableWow64FsRedirection" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!Wow64DisableWow64FsRedirection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Wow64GetThreadContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!Wow64GetThreadContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Wow64RevertWow64FsRedirection" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!Wow64RevertWow64FsRedirection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Wow64SetThreadContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!Wow64SetThreadContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Wow64Transition" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!Wow64Transition arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WriteConsoleA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!WriteConsoleA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WriteConsoleInputA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!WriteConsoleInputA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WriteConsoleInputW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!WriteConsoleInputW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WriteConsoleOutputA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "kernel32!WriteConsoleOutputA '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WriteConsoleOutputAttribute" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WriteConsoleOutputAttribute h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WriteConsoleOutputCharacterA" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_string(str_ptr);
            log_red!(
                emu,
                "kernel32!WriteConsoleOutputCharacterA '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WriteConsoleOutputCharacterW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(
                emu,
                "kernel32!WriteConsoleOutputCharacterW '{}'  0x{:x}",
                s,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WriteConsoleOutputW" => {
            let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "kernel32!WriteConsoleOutputW '{}'  0x{:x}", s, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WriteFileEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WriteFileEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WriteFileGather" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!WriteFileGather h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ZombifyActCtx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ZombifyActCtx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_hread" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!_hread arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_hwrite" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!_hwrite arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "lstrcmpi" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!lstrcmpi p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "lstrcpyA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!lstrcpyA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "lstrcpynA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!lstrcpynA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "lstrcpynW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "kernel32!lstrcpynW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "timeGetSystemTime" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!timeGetSystemTime arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateUmsCompletionList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "kernel32!CreateUmsCompletionList arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            let nparams = emu.banzai.get_params("CreateUmsCompletionList");
            if nparams > 0 {
                for _ in 0..nparams {
                    emu.stack_pop32(false);
                }
            }
            let hndl = helper::handler_create("kernel32://CreateUmsCompletionList");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateUmsThreadContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "kernel32!CreateUmsThreadContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            let nparams = emu.banzai.get_params("CreateUmsThreadContext");
            if nparams > 0 {
                for _ in 0..nparams {
                    emu.stack_pop32(false);
                }
            }
            let hndl = helper::handler_create("kernel32://CreateUmsThreadContext");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "DeleteUmsCompletionList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "kernel32!DeleteUmsCompletionList handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeleteUmsThreadContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "kernel32!DeleteUmsThreadContext handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DequeueUmsCompletionListItems" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!DequeueUmsCompletionListItems arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnterUmsSchedulingMode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!EnterUmsSchedulingMode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EtwEventEnabled" | "EventEnabled" => {
            let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let event_descriptor =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let level = if event_descriptor != 0 {
                emu.maps.read_byte(event_descriptor).unwrap_or(0)
            } else {
                0
            };
            log_red!(
                emu,
                "kernel32!EventEnabled handle:0x{:x} level:{}",
                reg_handle,
                level
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EtwEventRegister" | "EventRegister" => {
            let provider_guid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let callback = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let callback_ctx = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let reg_handle_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let guid_str = emu.maps.read_string(provider_guid);
            let hndl = helper::handler_create(&guid_str);
            if reg_handle_ptr != 0 {
                emu.maps.write_dword(reg_handle_ptr, hndl as u32);
            }
            log_red!(
                emu,
                "kernel32!EventRegister guid:'{}' cb:0x{:x} => handle:0x{:x}",
                guid_str,
                callback,
                hndl
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "EtwEventSetInformation" | "EventSetInformation" => {
            let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let info_class = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let info_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let info_data = emu.maps.read_string(info_ptr);
            log_red!(
                emu,
                "kernel32!EventSetInformation handle:0x{:x} class:{} data:'{}' len:{}",
                reg_handle,
                info_class,
                info_data,
                info_len
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "EtwEventWriteTransfer" | "EventWriteTransfer" => {
            let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let event_descriptor =
                emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let activity_id = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let related_id = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let desc_str = emu.maps.read_string(event_descriptor);
            log_red!(
                emu,
                "kernel32!EventWriteTransfer handle:0x{:x} desc:'{}' activity:0x{:x}",
                reg_handle,
                desc_str,
                activity_id
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "ExecuteUmsThread" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!ExecuteUmsThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentUmsThread" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetCurrentUmsThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNextUmsListItem" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetNextUmsListItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUmsCompletionListEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!GetUmsCompletionListEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetUmsSystemThreadInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetUmsSystemThreadInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUserDefaultLangId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!GetUserDefaultLangId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LStrCmpI" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!LStrCmpI arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LStrCmpIW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!LStrCmpIW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "QueryUmsThreadInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!QueryUmsThreadInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlAddFunctionTable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlAddFunctionTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlCompareMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlCompareMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlCopyMemory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlCopyMemory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlDeleteFunctionTable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlDeleteFunctionTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlExitUserProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlExitUserProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlInstallFunctionTableCallback" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlInstallFunctionTableCallback arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlIsEcCode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlIsEcCode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlLookupFunctionEntry" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlLookupFunctionEntry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlRaiseException" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlRaiseException arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlRestoreContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlRestoreContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlUnwindEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlUnwindEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlVirtualUnwind" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlVirtualUnwind arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RtlVirtualUnwind2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!RtlVirtualUnwind2 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUmsThreadInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!SetUmsThreadInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UmsThreadYield" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "kernel32!UmsThreadYield 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "__C_specific_handler" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!__C_specific_handler arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "__chkstk" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "kernel32!__chkstk arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "__misaligned_access" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!__misaligned_access arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_local_unwind" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!_local_unwind arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "uaw_lstrcmpW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!uaw_lstrcmpW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "uaw_lstrcmpiW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!uaw_lstrcmpiW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "uaw_lstrlenW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "kernel32!uaw_lstrlenW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "uaw_wcschr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!uaw_wcschr arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "uaw_wcscpy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!uaw_wcscpy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "uaw_wcsicmp" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!uaw_wcsicmp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "uaw_wcslen" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!uaw_wcslen arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "uaw_wcsrchr" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "kernel32!uaw_wcsrchr arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}
