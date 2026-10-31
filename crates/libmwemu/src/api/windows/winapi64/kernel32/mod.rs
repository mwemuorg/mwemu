use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::windows::structures;

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

        // --- String/Comparison ---
        "lstrcmpA" => {
            let s1 = emu.regs().rcx;
            let s2 = emu.regs().rdx;
            let str1 = emu.maps.read_string(s1);
            let str2 = emu.maps.read_string(s2);
            let r = str1.cmp(&str2) as i32;
            log_red!(emu, "kernel32!lstrcmpA '{}' '{}' => {}", str1, str2, r);
            emu.regs_mut().rax = r as u64;
        }
        "lstrcmpW" => {
            let s1 = emu.regs().rcx;
            let s2 = emu.regs().rdx;
            let str1 = emu.maps.read_wide_string(s1);
            let str2 = emu.maps.read_wide_string(s2);
            let r = str1.cmp(&str2) as i32;
            log_red!(emu, "kernel32!lstrcmpW '{}' '{}' => {}", str1, str2, r);
            emu.regs_mut().rax = r as u64;
        }
        "CompareStringA" => {
            let locale = emu.regs().rcx;
            let flags = emu.regs().rdx;
            let s1 = emu.regs().r8;
            let _len1 = emu.regs().r9;
            let str1 = emu.maps.read_string(s1);
            log_red!(
                emu,
                "kernel32!CompareStringA locale:{} flags:0x{:x} '{}'",
                locale,
                flags,
                str1
            );
            emu.regs_mut().rax = 2; // CSTR_EQUAL
        }
        "CompareStringOrdinal" => {
            let s1 = emu.regs().rcx;
            let len1 = emu.regs().rdx as i64;
            let s2 = emu.regs().r8;
            let _len2 = emu.regs().r9 as i64;
            let ignore_case = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let str1 = emu.maps.read_wide_string(s1);
            let str2 = emu.maps.read_wide_string(s2);
            let r = if ignore_case != 0 {
                str1.to_lowercase().cmp(&str2.to_lowercase()) as i32
            } else {
                str1.cmp(&str2) as i32
            };
            let result = match r.cmp(&0) {
                std::cmp::Ordering::Less => 1u64,
                std::cmp::Ordering::Equal => 2,
                std::cmp::Ordering::Greater => 3,
            };
            log_red!(
                emu,
                "kernel32!CompareStringOrdinal '{}' '{}' ic:{} => {}",
                str1,
                str2,
                ignore_case,
                result
            );
            emu.regs_mut().rax = result;
        }
        "GetStringTypeA" => {
            let locale = emu.regs().rcx;
            let dw_info_type = emu.regs().rdx;
            let len = emu.regs().r9;
            log_red!(
                emu,
                "kernel32!GetStringTypeA locale:{} type:{} len:{}",
                locale,
                dw_info_type,
                len
            );
            emu.regs_mut().rax = 1;
        }
        "GetOEMCP" => {
            log_red!(emu, "kernel32!GetOEMCP => 437");
            emu.regs_mut().rax = 437;
        }

        // --- Virtual Memory ---
        "VirtualQuery" => {
            let addr = emu.regs().rcx;
            let out_buff = emu.regs().rdx;
            let size = emu.regs().r8;
            log_red!(
                emu,
                "kernel32!VirtualQuery 0x{:x} 0x{:x} {}",
                addr,
                out_buff,
                size
            );
            if size < structures::MemoryBasicInformation64::SIZE {
                emu.regs_mut().rax = 0;
            } else {
                let mbi = structures::MemoryBasicInformation64::guess(addr, &mut emu.maps);
                mbi.save(out_buff, &mut emu.maps);
                emu.regs_mut().rax = structures::MemoryBasicInformation64::SIZE;
            }
        }
        "VirtualQueryEx" => {
            let _process = emu.regs().rcx;
            let addr = emu.regs().rdx;
            let out_buff = emu.regs().r8;
            let size = emu.regs().r9;
            log_red!(
                emu,
                "kernel32!VirtualQueryEx 0x{:x} 0x{:x} {}",
                addr,
                out_buff,
                size
            );
            if size < structures::MemoryBasicInformation64::SIZE {
                emu.regs_mut().rax = 0;
            } else {
                let mbi = structures::MemoryBasicInformation64::guess(addr, &mut emu.maps);
                mbi.save(out_buff, &mut emu.maps);
                emu.regs_mut().rax = structures::MemoryBasicInformation64::SIZE;
            }
        }

        // --- Path/File ---
        "GetLongPathNameA" => {
            let short = emu.regs().rcx;
            let long = emu.regs().rdx;
            let buf_len = emu.regs().r8 as u32;
            let path = emu.maps.read_string(short);
            log_red!(emu, "kernel32!GetLongPathNameA '{}'", path);
            if long != 0 && buf_len > path.len() as u32 {
                emu.maps.write_string(long, &path);
            }
            emu.regs_mut().rax = path.len() as u64;
        }
        "GetLongPathNameW" => {
            let short = emu.regs().rcx;
            let long = emu.regs().rdx;
            let buf_len = emu.regs().r8 as u32;
            let path = emu.maps.read_wide_string(short);
            log_red!(emu, "kernel32!GetLongPathNameW '{}'", path);
            if long != 0 && buf_len > path.len() as u32 {
                emu.maps.write_wide_string(long, &path);
            }
            emu.regs_mut().rax = path.len() as u64;
        }
        "GetShortPathNameA" => {
            let long = emu.regs().rcx;
            let short = emu.regs().rdx;
            let buf_len = emu.regs().r8 as u32;
            let path = emu.maps.read_string(long);
            log_red!(emu, "kernel32!GetShortPathNameA '{}'", path);
            if short != 0 && buf_len > path.len() as u32 {
                emu.maps.write_string(short, &path);
            }
            emu.regs_mut().rax = path.len() as u64;
        }
        "GetShortPathNameW" => {
            let long = emu.regs().rcx;
            let short = emu.regs().rdx;
            let buf_len = emu.regs().r8 as u32;
            let path = emu.maps.read_wide_string(long);
            log_red!(emu, "kernel32!GetShortPathNameW '{}'", path);
            if short != 0 && buf_len > path.len() as u32 {
                emu.maps.write_wide_string(short, &path);
            }
            emu.regs_mut().rax = path.len() as u64;
        }
        "SearchPathA" | "SearchPathW" => {
            log_red!(emu, "kernel32!{} (stub) => 0", api);
            emu.regs_mut().rax = 0;
        }
        "CopyFileExW" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            let src_s = emu.maps.read_wide_string(src);
            let dst_s = emu.maps.read_wide_string(dst);
            log_red!(emu, "kernel32!CopyFileExW '{}' -> '{}'", src_s, dst_s);
            emu.regs_mut().rax = 1;
        }
        "MoveFileExA" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            let src_s = emu.maps.read_string(src);
            let dst_s = emu.maps.read_string(dst);
            log_red!(emu, "kernel32!MoveFileExA '{}' -> '{}'", src_s, dst_s);
            emu.regs_mut().rax = 1;
        }
        "MoveFileExW" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            let src_s = emu.maps.read_wide_string(src);
            let dst_s = emu.maps.read_wide_string(dst);
            log_red!(emu, "kernel32!MoveFileExW '{}' -> '{}'", src_s, dst_s);
            emu.regs_mut().rax = 1;
        }
        "SetFilePointerEx" => {
            let handle = emu.regs().rcx;
            let distance = emu.regs().rdx;
            let new_pos_ptr = emu.regs().r8;
            let method = emu.regs().r9;
            log_red!(
                emu,
                "kernel32!SetFilePointerEx h:0x{:x} dist:{} method:{}",
                handle,
                distance as i64,
                method
            );
            if new_pos_ptr != 0 {
                emu.maps.write_qword(new_pos_ptr, distance);
            }
            emu.regs_mut().rax = 1;
        }
        "FlushFileBuffers" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "kernel32!FlushFileBuffers h:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "SetEndOfFile" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "kernel32!SetEndOfFile h:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "LockFile" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "kernel32!LockFile h:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "UnlockFile" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "kernel32!UnlockFile h:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CreatePipe" => {
            let read_pipe = emu.regs().rcx;
            let write_pipe = emu.regs().rdx;
            log_red!(emu, "kernel32!CreatePipe");
            if read_pipe != 0 {
                emu.maps
                    .write_qword(read_pipe, helper::handler_create("pipe_read"));
            }
            if write_pipe != 0 {
                emu.maps
                    .write_qword(write_pipe, helper::handler_create("pipe_write"));
            }
            emu.regs_mut().rax = 1;
        }
        "GetPrivateProfileStringA" => {
            let section = emu.regs().rcx;
            let key = emu.regs().rdx;
            let default = emu.regs().r8;
            let out_buf = emu.regs().r9;
            let buf_size = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let sec_s = if section != 0 {
                emu.maps.read_string(section)
            } else {
                String::new()
            };
            let key_s = if key != 0 {
                emu.maps.read_string(key)
            } else {
                String::new()
            };
            let def_s = if default != 0 {
                emu.maps.read_string(default)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "kernel32!GetPrivateProfileStringA [{}] {} def:'{}' => '{}'",
                sec_s,
                key_s,
                def_s,
                def_s
            );
            if out_buf != 0 && buf_size > 0 {
                let write_len = std::cmp::min(def_s.len(), (buf_size - 1) as usize);
                emu.maps.write_string(out_buf, &def_s[..write_len]);
            }
            emu.regs_mut().rax = def_s.len() as u64;
        }
        "GetPrivateProfileStringW" => {
            let section = emu.regs().rcx;
            let key = emu.regs().rdx;
            let default = emu.regs().r8;
            let out_buf = emu.regs().r9;
            let buf_size = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let sec_s = if section != 0 {
                emu.maps.read_wide_string(section)
            } else {
                String::new()
            };
            let key_s = if key != 0 {
                emu.maps.read_wide_string(key)
            } else {
                String::new()
            };
            let def_s = if default != 0 {
                emu.maps.read_wide_string(default)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "kernel32!GetPrivateProfileStringW [{}] {} => '{}'",
                sec_s,
                key_s,
                def_s
            );
            if out_buf != 0 && buf_size > 0 {
                emu.maps.write_wide_string(out_buf, &def_s);
            }
            emu.regs_mut().rax = def_s.len() as u64;
        }
        "WritePrivateProfileStringA" | "WritePrivateProfileStringW" => {
            log_red!(emu, "kernel32!{} (stub) => TRUE", api);
            emu.regs_mut().rax = 1;
        }
        "GetPrivateProfileIntA" | "GetPrivateProfileIntW" => {
            let _section = emu.regs().rcx;
            let _key = emu.regs().rdx;
            let default = emu.regs().r8;
            log_red!(emu, "kernel32!{} => default {}", api, default);
            emu.regs_mut().rax = default;
        }

        // --- Synchronization ---
        "CreateSemaphoreA" | "CreateSemaphoreW" => {
            let _security = emu.regs().rcx;
            let initial = emu.regs().rdx;
            let maximum = emu.regs().r8;
            let name = emu.regs().r9;
            let name_s = if name != 0 {
                emu.maps.read_string(name)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "kernel32!{} init:{} max:{} name:'{}'",
                api,
                initial,
                maximum,
                name_s
            );
            emu.regs_mut().rax = helper::handler_create(&name_s);
        }
        "CreateSemaphoreExW" => {
            let _security = emu.regs().rcx;
            let initial = emu.regs().rdx;
            let maximum = emu.regs().r8;
            let name = emu.regs().r9;
            let name_s = if name != 0 {
                emu.maps.read_wide_string(name)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "kernel32!CreateSemaphoreExW init:{} max:{} name:'{}'",
                initial,
                maximum,
                name_s
            );
            emu.regs_mut().rax = helper::handler_create(&name_s);
        }
        "CreateWaitableTimerA" | "CreateWaitableTimerW" => {
            let _security = emu.regs().rcx;
            let manual_reset = emu.regs().rdx;
            let name = emu.regs().r8;
            let name_s = if name != 0 {
                emu.maps.read_string(name)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "kernel32!{} manual:{} name:'{}'",
                api,
                manual_reset,
                name_s
            );
            emu.regs_mut().rax = helper::handler_create("waitable_timer");
        }
        "CreateWaitableTimerExW" => {
            let _security = emu.regs().rcx;
            let name = emu.regs().rdx;
            let name_s = if name != 0 {
                emu.maps.read_wide_string(name)
            } else {
                String::new()
            };
            log_red!(emu, "kernel32!CreateWaitableTimerExW name:'{}'", name_s);
            emu.regs_mut().rax = helper::handler_create("waitable_timer");
        }
        "OpenMutexA" => {
            let access = emu.regs().rcx;
            let inherit = emu.regs().rdx;
            let name = emu.regs().r8;
            let name_s = if name != 0 {
                emu.maps.read_string(name)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "kernel32!OpenMutexA access:0x{:x} name:'{}' => 0",
                access,
                name_s
            );
            emu.regs_mut().rax = 0;
            emu.set_last_error(2); // ERROR_FILE_NOT_FOUND
        }
        "OpenMutexW" => {
            let access = emu.regs().rcx;
            let inherit = emu.regs().rdx;
            let name = emu.regs().r8;
            let name_s = if name != 0 {
                emu.maps.read_wide_string(name)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "kernel32!OpenMutexW access:0x{:x} name:'{}' => 0",
                access,
                name_s
            );
            emu.regs_mut().rax = 0;
            emu.set_last_error(2);
        }
        "InterlockedIncrement" => {
            let addr = emu.regs().rcx;
            let val = emu.maps.read_dword(addr).unwrap_or(0).wrapping_add(1);
            emu.maps.write_dword(addr, val);
            log_red!(emu, "kernel32!InterlockedIncrement 0x{:x} => {}", addr, val);
            emu.regs_mut().rax = val as u64;
        }
        "InterlockedDecrement" => {
            let addr = emu.regs().rcx;
            let val = emu.maps.read_dword(addr).unwrap_or(0).wrapping_sub(1);
            emu.maps.write_dword(addr, val);
            log_red!(emu, "kernel32!InterlockedDecrement 0x{:x} => {}", addr, val);
            emu.regs_mut().rax = val as u64;
        }
        "InterlockedExchange" => {
            let addr = emu.regs().rcx;
            let new_val = emu.regs().rdx as u32;
            let old = emu.maps.read_dword(addr).unwrap_or(0);
            emu.maps.write_dword(addr, new_val);
            log_red!(
                emu,
                "kernel32!InterlockedExchange 0x{:x} {} => {}",
                addr,
                new_val,
                old
            );
            emu.regs_mut().rax = old as u64;
        }
        "InterlockedCompareExchange" => {
            let addr = emu.regs().rcx;
            let exchange = emu.regs().rdx as u32;
            let comparand = emu.regs().r8 as u32;
            let current = emu.maps.read_dword(addr).unwrap_or(0);
            if current == comparand {
                emu.maps.write_dword(addr, exchange);
            }
            log_red!(
                emu,
                "kernel32!InterlockedCompareExchange 0x{:x} exch:{} cmp:{} cur:{}",
                addr,
                exchange,
                comparand,
                current
            );
            emu.regs_mut().rax = current as u64;
        }
        "DeleteCriticalSection" => {
            let cs = emu.regs().rcx;
            log_red!(emu, "kernel32!DeleteCriticalSection 0x{:x}", cs);
        }
        "InitializeConditionVariable" => {
            let cv = emu.regs().rcx;
            log_red!(emu, "kernel32!InitializeConditionVariable 0x{:x}", cv);
            if cv != 0 {
                emu.maps.write_qword(cv, 0);
            }
        }
        "WakeConditionVariable" | "WakeAllConditionVariable" => {
            log_red!(emu, "kernel32!{} (stub)", api);
        }
        "SleepConditionVariableSRW" | "SleepConditionVariableCS" => {
            log_red!(emu, "kernel32!{} (stub) => TRUE", api);
            emu.regs_mut().rax = 1;
        }
        "SleepEx" => {
            let ms = emu.regs().rcx;
            let alertable = emu.regs().rdx;
            log_red!(emu, "kernel32!SleepEx ms:{} alertable:{}", ms, alertable);
            emu.regs_mut().rax = 0;
        }
        "WaitForSingleObjectEx" => {
            let handle = emu.regs().rcx;
            let ms = emu.regs().rdx;
            let alertable = emu.regs().r8;
            log_red!(
                emu,
                "kernel32!WaitForSingleObjectEx h:0x{:x} ms:{} alert:{}",
                handle,
                ms,
                alertable
            );
            emu.regs_mut().rax = 0; // WAIT_OBJECT_0
        }
        "WaitForMultipleObjectsEx" => {
            let count = emu.regs().rcx;
            let handles = emu.regs().rdx;
            let wait_all = emu.regs().r8;
            let ms = emu.regs().r9;
            log_red!(
                emu,
                "kernel32!WaitForMultipleObjectsEx count:{} wait_all:{} ms:{}",
                count,
                wait_all,
                ms
            );
            emu.regs_mut().rax = 0; // WAIT_OBJECT_0
        }

        // --- Process/Thread ---
        "GetExitCodeProcess" => {
            let handle = emu.regs().rcx;
            let exit_code_ptr = emu.regs().rdx;
            log_red!(emu, "kernel32!GetExitCodeProcess h:0x{:x}", handle);
            if exit_code_ptr != 0 {
                emu.maps.write_dword(exit_code_ptr, 259); // STILL_ACTIVE
            }
            emu.regs_mut().rax = 1;
        }
        "GetExitCodeThread" => {
            let handle = emu.regs().rcx;
            let exit_code_ptr = emu.regs().rdx;
            log_red!(emu, "kernel32!GetExitCodeThread h:0x{:x}", handle);
            if exit_code_ptr != 0 {
                emu.maps.write_dword(exit_code_ptr, 259); // STILL_ACTIVE
            }
            emu.regs_mut().rax = 1;
        }
        "GetProcessId" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "kernel32!GetProcessId h:0x{:x} => 1337", handle);
            emu.regs_mut().rax = 1337;
        }
        "SetThreadAffinityMask" => {
            let handle = emu.regs().rcx;
            let mask = emu.regs().rdx;
            log_red!(
                emu,
                "kernel32!SetThreadAffinityMask h:0x{:x} mask:0x{:x}",
                handle,
                mask
            );
            emu.regs_mut().rax = 1; // previous mask
        }
        "SetProcessAffinityMask" => {
            let handle = emu.regs().rcx;
            let mask = emu.regs().rdx;
            log_red!(
                emu,
                "kernel32!SetProcessAffinityMask h:0x{:x} mask:0x{:x}",
                handle,
                mask
            );
            emu.regs_mut().rax = 1;
        }
        "GetActiveProcessorCount" => {
            let group = emu.regs().rcx;
            let count: u64 = 4;
            log_red!(
                emu,
                "kernel32!GetActiveProcessorCount group:{} => {}",
                group,
                count
            );
            emu.regs_mut().rax = count;
        }
        "GetMaximumProcessorCount" => {
            let group = emu.regs().rcx;
            let count: u64 = 4;
            log_red!(
                emu,
                "kernel32!GetMaximumProcessorCount group:{} => {}",
                group,
                count
            );
            emu.regs_mut().rax = count;
        }
        "GetProcessWorkingSetSize" => {
            let handle = emu.regs().rcx;
            let min_ptr = emu.regs().rdx;
            let max_ptr = emu.regs().r8;
            log_red!(emu, "kernel32!GetProcessWorkingSetSize h:0x{:x}", handle);
            if min_ptr != 0 {
                emu.maps.write_qword(min_ptr, 0x20000);
            }
            if max_ptr != 0 {
                emu.maps.write_qword(max_ptr, 0x1400000);
            }
            emu.regs_mut().rax = 1;
        }
        "SetProcessWorkingSetSize" => {
            log_red!(emu, "kernel32!SetProcessWorkingSetSize (stub) => TRUE");
            emu.regs_mut().rax = 1;
        }
        "SetThreadContext" => {
            let handle = emu.regs().rcx;
            let context = emu.regs().rdx;
            log_red!(
                emu,
                "kernel32!SetThreadContext h:0x{:x} ctx:0x{:x} (stub)",
                handle,
                context
            );
            emu.regs_mut().rax = 1;
        }
        "SuspendThread" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "kernel32!SuspendThread h:0x{:x}", handle);
            emu.regs_mut().rax = 0; // previous suspend count
        }
        "Process32FirstW" => {
            let snap = emu.regs().rcx;
            let entry = emu.regs().rdx;
            log_red!(
                emu,
                "kernel32!Process32FirstW snap:0x{:x} entry:0x{:x} => FALSE",
                snap,
                entry
            );
            emu.regs_mut().rax = 0;
        }
        "Process32NextW" => {
            let snap = emu.regs().rcx;
            let entry = emu.regs().rdx;
            log_red!(
                emu,
                "kernel32!Process32NextW snap:0x{:x} entry:0x{:x} => FALSE",
                snap,
                entry
            );
            emu.regs_mut().rax = 0;
        }

        // --- Locale/NLS ---
        "GetUserDefaultUILanguage" => {
            log_red!(emu, "kernel32!GetUserDefaultUILanguage => 0x0409");
            emu.regs_mut().rax = 0x0409;
        }
        "SetLocaleInfoA" | "SetLocaleInfoW" => {
            log_red!(emu, "kernel32!{} (stub) => TRUE", api);
            emu.regs_mut().rax = 1;
        }
        "GetLocaleInfoEx" => {
            let locale_name = emu.regs().rcx;
            let lc_type = emu.regs().rdx;
            let buf = emu.regs().r8;
            let buf_len = emu.regs().r9;
            let name_s = if locale_name != 0 {
                emu.maps.read_wide_string(locale_name)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "kernel32!GetLocaleInfoEx '{}' type:0x{:x} buf_len:{}",
                name_s,
                lc_type,
                buf_len
            );
            if buf != 0 && buf_len > 0 {
                emu.maps.write_wide_string(buf, "en-US");
                emu.regs_mut().rax = 6; // length of "en-US" + null
            } else {
                emu.regs_mut().rax = 6;
            }
        }
        "GetDateFormatW" => {
            let locale = emu.regs().rcx;
            let flags = emu.regs().rdx;
            let _date = emu.regs().r8;
            let _fmt = emu.regs().r9;
            let buf = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let buf_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let date_str = "01/01/2024";
            log_red!(
                emu,
                "kernel32!GetDateFormatW locale:{} flags:0x{:x}",
                locale,
                flags
            );
            if buf != 0 && buf_len > 0 {
                emu.maps.write_wide_string(buf, date_str);
            }
            emu.regs_mut().rax = (date_str.len() + 1) as u64;
        }
        "GetTimeFormatW" => {
            let locale = emu.regs().rcx;
            let flags = emu.regs().rdx;
            let _time = emu.regs().r8;
            let _fmt = emu.regs().r9;
            let buf = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let buf_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let time_str = "00:00:00";
            log_red!(
                emu,
                "kernel32!GetTimeFormatW locale:{} flags:0x{:x}",
                locale,
                flags
            );
            if buf != 0 && buf_len > 0 {
                emu.maps.write_wide_string(buf, time_str);
            }
            emu.regs_mut().rax = (time_str.len() + 1) as u64;
        }

        // --- Atom table ---
        "GlobalAddAtomW" => {
            let name = emu.regs().rcx;
            let name_s = if name != 0 {
                emu.maps.read_wide_string(name)
            } else {
                String::new()
            };
            log_red!(emu, "kernel32!GlobalAddAtomW '{}' => 0xC001", name_s);
            emu.regs_mut().rax = 0xC001;
        }
        "GlobalDeleteAtom" => {
            let atom = emu.regs().rcx;
            log_red!(emu, "kernel32!GlobalDeleteAtom atom:0x{:x}", atom);
            emu.regs_mut().rax = 0;
        }
        "GlobalFindAtomA" => {
            let name = emu.regs().rcx;
            let name_s = if name != 0 {
                emu.maps.read_string(name)
            } else {
                String::new()
            };
            log_red!(emu, "kernel32!GlobalFindAtomA '{}' => 0", name_s);
            emu.regs_mut().rax = 0;
            emu.set_last_error(2);
        }
        "GlobalFindAtomW" => {
            let name = emu.regs().rcx;
            let name_s = if name != 0 {
                emu.maps.read_wide_string(name)
            } else {
                String::new()
            };
            log_red!(emu, "kernel32!GlobalFindAtomW '{}' => 0", name_s);
            emu.regs_mut().rax = 0;
            emu.set_last_error(2);
        }
        "GlobalGetAtomNameA" | "GlobalGetAtomNameW" => {
            let atom = emu.regs().rcx;
            log_red!(emu, "kernel32!{} atom:0x{:x} => 0", api, atom);
            emu.regs_mut().rax = 0;
        }
        "GlobalFlags" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "kernel32!GlobalFlags h:0x{:x} => 0", handle);
            emu.regs_mut().rax = 0;
        }
        "GlobalSize" => {
            let handle = emu.regs().rcx;
            let sz = match emu.maps.get_mem_by_addr(handle) {
                Some(mem) => mem.size() as u64,
                None => 0,
            };
            log_red!(emu, "kernel32!GlobalSize h:0x{:x} => {}", handle, sz);
            emu.regs_mut().rax = sz;
        }
        "GlobalReAlloc" => {
            let handle = emu.regs().rcx;
            let size = emu.regs().rdx;
            let flags = emu.regs().r8;
            log_red!(
                emu,
                "kernel32!GlobalReAlloc h:0x{:x} sz:{} flags:0x{:x} => h",
                handle,
                size,
                flags
            );
            emu.regs_mut().rax = handle; // simplified: return same handle
        }

        // --- Environment ---
        "GetEnvironmentStrings" | "GetEnvironmentStringsA" => {
            log_red!(emu, "kernel32!GetEnvironmentStrings");
            let block = emu.maps.alloc(0x100).unwrap_or(0);
            if block != 0 {
                let _ = emu.maps.create_map(
                    "env_block",
                    block,
                    0x100,
                    crate::maps::mem64::Permission::READ_WRITE,
                );
                emu.maps.write_byte(block, 0);
                emu.maps.write_byte(block + 1, 0);
            }
            emu.regs_mut().rax = block;
        }

        // --- Misc ---
        "Beep" => {
            let freq = emu.regs().rcx;
            let duration = emu.regs().rdx;
            log_red!(emu, "kernel32!Beep freq:{} dur:{}", freq, duration);
            emu.regs_mut().rax = 1;
        }
        "GetSystemWindowsDirectoryA" => {
            let buf = emu.regs().rcx;
            let size = emu.regs().rdx;
            let path = "C:\\Windows";
            log_red!(emu, "kernel32!GetSystemWindowsDirectoryA => '{}'", path);
            if buf != 0 && size > path.len() as u64 {
                emu.maps.write_string(buf, path);
            }
            emu.regs_mut().rax = path.len() as u64;
        }
        "GetSystemWindowsDirectoryW" => {
            let buf = emu.regs().rcx;
            let size = emu.regs().rdx;
            let path = "C:\\Windows";
            log_red!(emu, "kernel32!GetSystemWindowsDirectoryW => '{}'", path);
            if buf != 0 && size > path.len() as u64 {
                emu.maps.write_wide_string(buf, path);
            }
            emu.regs_mut().rax = path.len() as u64;
        }
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
