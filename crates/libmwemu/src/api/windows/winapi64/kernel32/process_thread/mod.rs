pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    CheckRemoteDebuggerPresent, CreateProcessA, CreateProcessW, CreateRemoteThread,
    DuplicateHandle, GetThreadLocale, IsDebuggerPresent, IsProcessorFeaturePresent, IsWow64Process,
    RaiseException, SetThreadLocale, SetThreadStackGuarantee, TerminateProcess,
    UnhandledExceptionFilter,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::libraries::kernel32::system::{
    GetActiveProcessorGroupCount, GetMaximumProcessorGroupCount, GetThreadUILanguage,
    SetThreadUILanguage,
};
use crate::winapi::winapi64::kernel32;

pub use crate::api::windows::common::process::GetCurrentProcessId;
pub use crate::api::windows::common::threads::{GetCurrentThreadId, GetThreadId, OpenThread};
pub use crate::api::windows::common::toolhelp::{
    CreateToolhelp32Snapshot, Module32First, Module32FirstW, Module32Next, Module32NextW,
    Process32First, Process32FirstW, Process32Next, Process32NextW, Thread32First, Thread32Next,
};

pub use crate::api::windows::libraries::kernel32::process_thread::{
    CreateThread, ExitThread, GetProcessAffinityMask, GetThreadContext, ResumeThread,
};

pub use crate::winapi::common::threads::AddVectoredExceptionHandler;

mod exit_process;
pub use exit_process::ExitProcess;

pub use crate::winapi::common::security::OpenProcessToken;

mod set_unhandled_exception_filter;
pub use set_unhandled_exception_filter::SetUnhandledExceptionFilter;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GetActiveProcessorGroupCount" => GetActiveProcessorGroupCount(emu),
        "GetMaximumProcessorGroupCount" => GetMaximumProcessorGroupCount(emu),
        "GetThreadUILanguage" => GetThreadUILanguage(emu),
        "SetThreadUILanguage" => SetThreadUILanguage(emu),
        "CreateProcessA" => kernel32::CreateProcessA(emu),
        "CreateProcessW" => kernel32::CreateProcessW(emu),
        "CreateRemoteThread" => kernel32::CreateRemoteThread(emu),
        "CreateThread" => kernel32::CreateThread(emu),
        "CreateToolhelp32Snapshot" => kernel32::CreateToolhelp32Snapshot(emu),
        "ExitProcess" => kernel32::ExitProcess(emu),
        "ExitThread" => kernel32::ExitThread(emu),
        "RtlExitUserProcess" => kernel32::RtlExitUserProcess(emu),
        "FlsAlloc" => kernel32::FlsAlloc(emu),
        "FlsFree" => kernel32::FlsFree(emu),
        "FlsGetValue" => kernel32::FlsGetValue(emu),
        "FlsGetValue2" => kernel32::FlsGetValue2(emu),
        "FlsSetValue" => kernel32::FlsSetValue(emu),
        "GetCurrentProcessId" => kernel32::GetCurrentProcessId(emu),
        "GetCurrentThreadId" => kernel32::GetCurrentThreadId(emu),
        "GetProcessAffinityMask" => kernel32::GetProcessAffinityMask(emu),
        "GetStartupInfoA" => kernel32::GetStartupInfoA(emu),
        "GetStartupInfoW" => kernel32::GetStartupInfoW(emu),
        "GetThreadId" => kernel32::GetThreadId(emu),
        "GetThreadContext" => kernel32::GetThreadContext(emu),
        "GetThreadLocale" => kernel32::GetThreadLocale(emu),
        "IsWow64Process" => kernel32::IsWow64Process(emu),
        "IsProcessorFeaturePresent" => kernel32::IsProcessorFeaturePresent(emu),
        "OpenThread" => kernel32::OpenThread(emu),
        "Process32First" => kernel32::Process32First(emu),
        "Process32Next" => kernel32::Process32Next(emu),
        "ResumeThread" => kernel32::ResumeThread(emu),
        "SetThreadLocale" => kernel32::SetThreadLocale(emu),
        "SetThreadStackGuarantee" => kernel32::SetThreadStackGuarantee(emu),
        "TerminateProcess" => kernel32::TerminateProcess(emu),
        "Thread32First" => kernel32::Thread32First(emu),
        "Thread32Next" => kernel32::Thread32Next(emu),
        "TlsAlloc" => kernel32::TlsAlloc(emu),
        "TlsFree" => kernel32::TlsFree(emu),
        "TlsGetValue" => kernel32::TlsGetValue(emu),
        "TlsGetValue2" => kernel32::TlsGetValue2(emu),
        "TlsSetValue" => kernel32::TlsSetValue(emu),
        "WinExec" => kernel32::WinExec(emu),
        "SetThreadAffinityMask" => kernel32::SetThreadAffinityMask(emu),
        "SetProcessAffinityMask" => kernel32::SetProcessAffinityMask(emu),
        "GetActiveProcessorCount" => kernel32::GetActiveProcessorCount(emu),
        "GetMaximumProcessorCount" => kernel32::GetMaximumProcessorCount(emu),
        "SetThreadContext" => kernel32::SetThreadContext(emu),
        "SuspendThread" => kernel32::SuspendThread(emu),
        "Process32FirstW" => kernel32::Process32FirstW(emu),
        "Process32NextW" => kernel32::Process32NextW(emu),
        "OpenProcessToken" => kernel32::OpenProcessToken(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
