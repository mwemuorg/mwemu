use crate::emu;
use crate::serialization;

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

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
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
        "EtwEventEnabled" | "EventEnabled" => {
            log_red!(emu, "kernel32!EtwEventEnabled (stub)");
            emu.regs_mut().rax = 0;
        }
        "EtwEventRegister" | "EventRegister" => {
            log_red!(emu, "kernel32!EtwEventRegister (stub)");
            emu.regs_mut().rax = 0;
        }
        "EtwEventSetInformation" | "EventSetInformation" => {
            log_red!(emu, "kernel32!EtwEventSetInformation (stub)");
            emu.regs_mut().rax = 0;
        }
        "EtwEventWriteTransfer" | "EventWriteTransfer" => {
            log_red!(emu, "kernel32!EtwEventWriteTransfer (stub)");
            emu.regs_mut().rax = 0;
        }
        "ExitProcess" => ExitProcess(emu),
        "ExitThread" => ExitThread(emu),
        "RtlExitUserProcess" => {
            log_red!(emu, "ntdll!RtlExitUserProcess");
            emu.stop();
        }
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
        "FlsFree" => {
            let idx = emu.regs().rcx;
            log_red!(emu, "kernel32!FlsFree idx: {}", idx);
            if (idx as usize) < emu.fls().len() {
                emu.fls_mut()[idx as usize] = 0;
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "FlsGetValue" => FlsGetValue(emu),
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
        "GetEnvironmentVariableW" => GetEnvironmentVariableW(emu),
        "GetFileAttributesA" => GetFileAttributesA(emu),
        "GetFileAttributesW" => GetFileAttributesW(emu),
        "GetFileSize" => GetFileSize(emu),
        "GetFileSizeEx" => GetFileSizeEx(emu),
        "GetFileType" => GetFileType(emu),
        "GetFullPathNameA" => GetFullPathNameA(emu),
        "GetFullPathNameW" => GetFullPathNameW(emu),
        "GetCurrentPackagePath" => {
            log_red!(emu, "kernel32!GetCurrentPackagePath (stub)");
            emu.regs_mut().rax = 0x80073D54; // APPMODEL_ERROR_NO_PACKAGE
        }
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
        "GetStringTypeW" => {
            let dw_info_type = emu.regs().rcx;
            let len = emu.regs().r8;
            log_red!(
                emu,
                "kernel32!GetStringTypeW type: {} len: {}",
                dw_info_type,
                len
            );
            emu.regs_mut().rax = 1;
        }
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
        "IsDebuggerPresent" => IsDebuggerPresent(emu),
        "IsValidCodePage" => {
            let cp = emu.regs().rcx;
            log_red!(emu, "kernel32!IsValidCodePage cp: {}", cp);
            emu.regs_mut().rax = 1;
        }
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
        "LCMapStringW" => {
            let locale = emu.regs().rcx;
            let flags = emu.regs().rdx;
            let src = emu.regs().r8;
            let src_len = emu.regs().r9 as i64;
            let dst = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let dst_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i64;
            let real_src_len = if src_len < 0 {
                let s = emu.maps.read_wide_string(src);
                (s.len() + 1) as i64
            } else {
                src_len
            };
            log_red!(
                emu,
                "kernel32!LCMapStringW locale: 0x{:x} flags: 0x{:x} src_len: {} dst_len: {}",
                locale,
                flags,
                real_src_len,
                dst_len
            );
            if dst == 0 || dst_len == 0 {
                emu.regs_mut().rax = real_src_len as u64;
            } else {
                let copy_len = std::cmp::min(real_src_len, dst_len) as u64;
                for i in 0..copy_len * 2 {
                    let b = emu.maps.read_byte(src + i).unwrap_or(0);
                    emu.maps.write_byte(dst + i, b);
                }
                emu.regs_mut().rax = copy_len;
            }
        }
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
        "RegCloseKey" => RegCloseKey(emu),
        "RegCreateKeyExA" => RegCreateKeyExA(emu),
        "RegCreateKeyExW" => RegCreateKeyExW(emu),
        "RegOpenKeyA" => RegOpenKeyA(emu),
        "RegSetValueExA" => RegSetValueExA(emu),
        "RegSetValueExW" => RegSetValueExW(emu),
        "ResetEvent" => ResetEvent(emu),
        "SetEvent" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "kernel32!SetEvent handle: 0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }

        "ReleaseSRWLockExclusive" => ReleaseSRWLockExclusive(emu),
        "ReleaseSRWLockShared" => ReleaseSRWLockShared(emu),
        "ResumeThread" => ResumeThread(emu),
        "SetCurrentDirectoryA" => SetCurrentDirectoryA(emu),
        "SetEnvironmentVariableA" => SetEnvironmentVariableA(emu),
        "SetEnvironmentVariableW" => SetEnvironmentVariableW(emu),
        "SetErrorMode" => SetErrorMode(emu),
        "SetHandleCount" => {
            let count = emu.regs().rcx;
            log_red!(emu, "kernel32!SetHandleCount count: {}", count);
            emu.regs_mut().rax = count;
        }
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
        "VerifyVersionInfoA" | "VerifyVersionInfoW" => {
            log_red!(emu, "kernel32!VerifyVersionInfo (stub) -> TRUE");
            emu.regs_mut().rax = 1;
        }
        "VerSetConditionMask" => {
            let mask = emu.regs().rcx;
            let type_bit = emu.regs().rdx;
            let condition = emu.regs().r8;
            log_red!(
                emu,
                "kernel32!VerSetConditionMask mask: 0x{:x} type: {} cond: {}",
                mask,
                type_bit,
                condition
            );
            emu.regs_mut().rax = mask | (condition << (type_bit & 0x1f));
        }
        "WideCharToMultiByte" => WideCharToMultiByte(emu),
        "WinExec" => WinExec(emu),
        "WriteConsoleW" => WriteConsoleW(emu),
        "WriteFile" => WriteFile(emu),
        "WriteProcessMemory" => WriteProcessMemory(emu),
        "GetVolumeInformationA" => GetVolumeInformationA(emu),
        "GetVolumeInformationW" => GetVolumeInformationW(emu),
        "RemoveDirectoryA" => RemoveDirectoryA(emu),
        "RemoveDirectoryW" => RemoveDirectoryW(emu),
        "SetFileAttributesA" => SetFileAttributesA(emu),
        "SetFileAttributesW" => SetFileAttributesW(emu),
        "DeviceIoControl" => api_DeviceIoControl(emu),
        _ => {
            // Windows forwards many Rtl*/Nt* functions through kernel32 →
            // ntdll. Try ntdll sub-dispatchers before giving up.
            if super::ntdll::try_dispatch(api, emu) {
                return String::new();
            }

            if emu.cfg.skip_unimplemented {
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} at 0x{:x}",
                    addr,
                    api,
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
