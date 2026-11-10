use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

#[path = "../create_process.rs"]
mod create_process;
pub use create_process::*;

#[path = "../add_vectored_exception_handler.rs"]
mod add_vectored_exception_handler;
pub use add_vectored_exception_handler::*;

#[path = "../create_remote_thread.rs"]
mod create_remote_thread;
pub use create_remote_thread::*;

#[path = "../create_thread.rs"]
mod create_thread;
pub use create_thread::*;

#[path = "../create_toolhelp32_snapshot.rs"]
mod create_toolhelp32_snapshot;
pub use create_toolhelp32_snapshot::*;

#[path = "../module32_first.rs"]
mod module32_first;
pub use module32_first::*;

#[path = "../module32_first_w.rs"]
mod module32_first_w;
pub use module32_first_w::*;

#[path = "../module32_next.rs"]
mod module32_next;
pub use module32_next::*;

#[path = "../module32_next_w.rs"]
mod module32_next_w;
pub use module32_next_w::*;

#[path = "../exit_process.rs"]
mod exit_process;
pub use exit_process::*;

#[path = "../get_current_process_id.rs"]
mod get_current_process_id;
pub use get_current_process_id::*;

#[path = "../get_current_thread_id.rs"]
mod get_current_thread_id;
pub use get_current_thread_id::*;

#[path = "../get_process_affinity_mask.rs"]
mod get_process_affinity_mask;
pub use get_process_affinity_mask::*;

#[path = "../get_thread_context.rs"]
mod get_thread_context;
pub use get_thread_context::*;

#[path = "../get_thread_locale.rs"]
mod get_thread_locale;
pub use get_thread_locale::*;

#[path = "../check_remote_debugger_present.rs"]
mod check_remote_debugger_present;
pub use check_remote_debugger_present::*;

#[path = "../is_debugger_present.rs"]
mod is_debugger_present;
pub use is_debugger_present::*;

#[path = "../is_processor_feature_present.rs"]
mod is_processor_feature_present;
pub use is_processor_feature_present::*;

#[path = "../open_process.rs"]
mod open_process;
pub use open_process::*;

#[path = "../open_process_token.rs"]
mod open_process_token;
pub use open_process_token::*;

#[path = "../open_thread.rs"]
mod open_thread;
pub use open_thread::*;

#[path = "../process32_first.rs"]
mod process32_first;
pub use process32_first::*;

#[path = "../process32_next.rs"]
mod process32_next;
pub use process32_next::*;

#[path = "../get_thread_id.rs"]
mod get_thread_id;
pub use get_thread_id::*;

#[path = "../resume_thread.rs"]
mod resume_thread;
pub use resume_thread::*;

#[path = "../set_thread_locale.rs"]
mod set_thread_locale;
pub use set_thread_locale::*;

#[path = "../set_thread_stack_guarantee.rs"]
mod set_thread_stack_guarantee;
pub use set_thread_stack_guarantee::*;

#[path = "../raise_exception.rs"]
mod raise_exception;
pub use raise_exception::*;

#[path = "../set_unhandled_exception_filter.rs"]
mod set_unhandled_exception_filter;
pub use set_unhandled_exception_filter::*;

#[path = "../terminate_process.rs"]
mod terminate_process;
pub use terminate_process::*;

#[path = "../thread32_first.rs"]
mod thread32_first;
pub use thread32_first::*;

#[path = "../thread32_next.rs"]
mod thread32_next;
pub use thread32_next::*;

#[path = "../unhandled_exception_filter.rs"]
mod unhandled_exception_filter;
pub use unhandled_exception_filter::*;

#[path = "../exit_thread.rs"]
mod exit_thread;
pub use exit_thread::*;

#[path = "../is_wow64_process.rs"]
mod is_wow64_process;
pub use is_wow64_process::*;

#[path = "../duplicate_handle.rs"]
mod duplicate_handle;
pub use duplicate_handle::*;

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
