use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod create_process;
pub use create_process::{CreateProcessA, CreateProcessW};

mod add_vectored_exception_handler;
pub use add_vectored_exception_handler::AddVectoredExceptionHandler;

mod create_remote_thread;
pub use create_remote_thread::CreateRemoteThread;

mod create_thread;
pub use create_thread::CreateThread;

mod create_toolhelp32_snapshot;
pub use create_toolhelp32_snapshot::CreateToolhelp32Snapshot;

mod module32_first;
pub use module32_first::{Module32First, Module32FirstW};

mod module32_next;
pub use module32_next::{Module32Next, Module32NextW};

mod exit_process;
pub use exit_process::ExitProcess;

mod get_current_process_id;
pub use get_current_process_id::GetCurrentProcessId;

mod get_current_thread_id;
pub use get_current_thread_id::GetCurrentThreadId;

mod get_process_affinity_mask;
pub use get_process_affinity_mask::GetProcessAffinityMask;

mod get_thread_context;
pub use get_thread_context::GetThreadContext;

mod get_thread_locale;
pub use get_thread_locale::GetThreadLocale;

mod check_remote_debugger_present;
pub use check_remote_debugger_present::CheckRemoteDebuggerPresent;

mod is_debugger_present;
pub use is_debugger_present::IsDebuggerPresent;

mod is_processor_feature_present;
pub use is_processor_feature_present::IsProcessorFeaturePresent;

mod open_process;
pub use open_process::OpenProcess;

mod open_process_token;
pub use open_process_token::OpenProcessToken;

mod open_thread;
pub use open_thread::OpenThread;

mod process32_first;
pub use process32_first::{Process32First, Process32FirstW};

mod process32_next;
pub use process32_next::{Process32Next, Process32NextW};

mod get_thread_id;
pub use get_thread_id::GetThreadId;

mod resume_thread;
pub use resume_thread::ResumeThread;

mod set_thread_locale;
pub use set_thread_locale::SetThreadLocale;

mod set_thread_stack_guarantee;
pub use set_thread_stack_guarantee::SetThreadStackGuarantee;

mod raise_exception;
pub use raise_exception::RaiseException;

mod set_unhandled_exception_filter;
pub use set_unhandled_exception_filter::SetUnhandledExceptionFilter;

mod terminate_process;
pub use terminate_process::TerminateProcess;

mod thread32_first;
pub use thread32_first::Thread32First;

mod thread32_next;
pub use thread32_next::Thread32Next;

mod unhandled_exception_filter;
pub use unhandled_exception_filter::UnhandledExceptionFilter;

mod exit_thread;
pub use exit_thread::ExitThread;

mod is_wow64_process;
pub use is_wow64_process::IsWow64Process;

mod duplicate_handle;
pub use duplicate_handle::DuplicateHandle;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "CreateProcessA" => kernel32::CreateProcessA(emu),
        "CreateProcessW" => kernel32::CreateProcessW(emu),
        "CreateRemoteThread" => kernel32::CreateRemoteThread(emu),
        "CreateThread" => kernel32::CreateThread(emu),
        "CreateToolhelp32Snapshot" => kernel32::CreateToolhelp32Snapshot(emu),
        "ExitProcess" => kernel32::ExitProcess(emu),
        "ExitThread" => kernel32::ExitThread(emu),
        "RtlExitUserProcess" => kernel32::RtlExitUserProcess(emu),
        "ExpandEnvironmentStringsA" => kernel32::ExpandEnvironmentStringsA(emu),
        "ExpandEnvironmentStringsW" => kernel32::ExpandEnvironmentStringsW(emu),
        "FlsAlloc" => kernel32::FlsAlloc(emu),
        "FlsFree" => kernel32::FlsFree(emu),
        "FlsGetValue" => kernel32::FlsGetValue(emu),
        "FlsGetValue2" => kernel32::FlsGetValue2(emu),
        "FlsSetValue" => kernel32::FlsSetValue(emu),
        "GetCurrentProcessId" => kernel32::GetCurrentProcessId(emu),
        "GetCurrentThreadId" => kernel32::GetCurrentThreadId(emu),
        "GetEnvironmentStringsW" => kernel32::GetEnvironmentStringsW(emu),
        "FreeEnvironmentStringsA" => kernel32::FreeEnvironmentStrings(emu),
        "FreeEnvironmentStringsW" => kernel32::FreeEnvironmentStrings(emu),
        "GetEnvironmentVariableA" => kernel32::GetEnvironmentVariableA(emu),
        "GetEnvironmentVariableW" => kernel32::GetEnvironmentVariableW(emu),
        "GetProcessAffinityMask" => kernel32::GetProcessAffinityMask(emu),
        "GetStartupInfoA" => kernel32::GetStartupInfoA(emu),
        "GetStartupInfoW" => kernel32::GetStartupInfoW(emu),
        "GetThreadId" => kernel32::GetThreadId(emu),
        "GetThreadContext" => kernel32::GetThreadContext(emu),
        "GetThreadLocale" => kernel32::GetThreadLocale(emu),
        "IsWow64Process" => kernel32::IsWow64Process(emu),
        "IsProcessorFeaturePresent" => kernel32::IsProcessorFeaturePresent(emu),
        "OpenProcess" => kernel32::OpenProcess(emu),
        "OpenThread" => kernel32::OpenThread(emu),
        "Process32First" => kernel32::Process32First(emu),
        "Process32Next" => kernel32::Process32Next(emu),
        "ResumeThread" => kernel32::ResumeThread(emu),
        "SetEnvironmentVariableA" => kernel32::SetEnvironmentVariableA(emu),
        "SetEnvironmentVariableW" => kernel32::SetEnvironmentVariableW(emu),
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
        "GetExitCodeProcess" => kernel32::GetExitCodeProcess(emu),
        "GetExitCodeThread" => kernel32::GetExitCodeThread(emu),
        "GetProcessId" => kernel32::GetProcessId(emu),
        "SetThreadAffinityMask" => kernel32::SetThreadAffinityMask(emu),
        "SetProcessAffinityMask" => kernel32::SetProcessAffinityMask(emu),
        "GetActiveProcessorCount" => kernel32::GetActiveProcessorCount(emu),
        "GetMaximumProcessorCount" => kernel32::GetMaximumProcessorCount(emu),
        "SetThreadContext" => kernel32::SetThreadContext(emu),
        "SuspendThread" => kernel32::SuspendThread(emu),
        "Process32FirstW" => kernel32::Process32FirstW(emu),
        "Process32NextW" => kernel32::Process32NextW(emu),
        "GetEnvironmentStrings" | "GetEnvironmentStringsA" => kernel32::GetEnvironmentStringsA(emu),
        "OpenProcessToken" => kernel32::OpenProcessToken(emu),
        _ => return false,
    }
    true
}
