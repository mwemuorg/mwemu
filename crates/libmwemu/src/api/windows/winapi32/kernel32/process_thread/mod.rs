use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

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

#[path = "../module32_next.rs"]
mod module32_next;
pub use module32_next::*;

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

#[path = "../get_thread_preferred_ui_languages.rs"]
mod get_thread_preferred_ui_languages;
pub use get_thread_preferred_ui_languages::*;

#[path = "../get_thread_ui_language.rs"]
mod get_thread_ui_language;
pub use get_thread_ui_language::*;

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

#[path = "../raise_exception.rs"]
mod raise_exception;
pub use raise_exception::*;

#[path = "../get_thread_id.rs"]
mod get_thread_id;
pub use get_thread_id::*;

#[path = "../resume_thread.rs"]
mod resume_thread;
pub use resume_thread::*;

#[path = "../set_thread_context.rs"]
mod set_thread_context;
pub use set_thread_context::*;

#[path = "../set_thread_locale.rs"]
mod set_thread_locale;
pub use set_thread_locale::*;

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

#[path = "../debug_set_process_kill_on_exit.rs"]
mod debug_set_process_kill_on_exit;
pub use debug_set_process_kill_on_exit::*;

#[path = "../disable_thread_profiling.rs"]
mod disable_thread_profiling;
pub use disable_thread_profiling::*;

#[path = "../enable_thread_profiling.rs"]
mod enable_thread_profiling;
pub use enable_thread_profiling::*;

#[path = "../get_active_processor_count.rs"]
mod get_active_processor_count;
pub use get_active_processor_count::*;

#[path = "../get_active_processor_group_count.rs"]
mod get_active_processor_group_count;
pub use get_active_processor_group_count::*;

#[path = "../get_exit_code_process_implementation.rs"]
mod get_exit_code_process_implementation;
pub use get_exit_code_process_implementation::*;

#[path = "../get_maximum_processor_count.rs"]
mod get_maximum_processor_count;
pub use get_maximum_processor_count::*;

#[path = "../get_maximum_processor_group_count.rs"]
mod get_maximum_processor_group_count;
pub use get_maximum_processor_group_count::*;

#[path = "../get_named_pipe_client_process_id.rs"]
mod get_named_pipe_client_process_id;
pub use get_named_pipe_client_process_id::*;

#[path = "../get_numa_node_processor_mask.rs"]
mod get_numa_node_processor_mask;
pub use get_numa_node_processor_mask::*;

#[path = "../get_numa_processor_node.rs"]
mod get_numa_processor_node;
pub use get_numa_processor_node::*;

#[path = "../get_numa_processor_node_ex.rs"]
mod get_numa_processor_node_ex;
pub use get_numa_processor_node_ex::*;

#[path = "../get_process_dep_policy.rs"]
mod get_process_dep_policy;
pub use get_process_dep_policy::*;

#[path = "../get_process_io_counters.rs"]
mod get_process_io_counters;
pub use get_process_io_counters::*;

#[path = "../get_process_working_set_size.rs"]
mod get_process_working_set_size;
pub use get_process_working_set_size::*;

#[path = "../get_thread_selector_entry.rs"]
mod get_thread_selector_entry;
pub use get_thread_selector_entry::*;

#[path = "../is_thread_a_fiber.rs"]
mod is_thread_a_fiber;
pub use is_thread_a_fiber::*;

#[path = "../process32_next.rs"]
mod process32_next;
pub use process32_next::*;

#[path = "../query_thread_profiling.rs"]
mod query_thread_profiling;
pub use query_thread_profiling::*;

#[path = "../read_thread_profiling_data.rs"]
mod read_thread_profiling_data;
pub use read_thread_profiling_data::*;

#[path = "../set_process_affinity_mask.rs"]
mod set_process_affinity_mask;
pub use set_process_affinity_mask::*;

#[path = "../set_process_dep_policy.rs"]
mod set_process_dep_policy;
pub use set_process_dep_policy::*;

#[path = "../set_process_working_set_size.rs"]
mod set_process_working_set_size;
pub use set_process_working_set_size::*;

#[path = "../set_thread_affinity_mask.rs"]
mod set_thread_affinity_mask;
pub use set_thread_affinity_mask::*;

#[path = "../set_thread_execution_state.rs"]
mod set_thread_execution_state;
pub use set_thread_execution_state::*;

#[path = "../exit_thread.rs"]
mod exit_thread;
pub use exit_thread::*;

#[path = "../process32_first.rs"]
mod process32_first;
pub use process32_first::*;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "IsWow64Process" => kernel32::IsWow64Process(emu),
        "ExitThread" => kernel32::ExitThread(emu),
        "CreateProcessW" => kernel32::CreateProcessW(emu),
        "Process32First" => kernel32::Process32First(emu),
        "GetThreadLocale" => kernel32::GetThreadLocale(emu),
        "SetEnvironmentVariableA" => kernel32::SetEnvironmentVariableA(emu),
        "SetEnvironmentVariableW" => kernel32::SetEnvironmentVariableW(emu),
        "GetEnvironmentVariableW" => kernel32::GetEnvironmentVariableW(emu),
        "CreateProcessA" => kernel32::CreateProcessA(emu),
        "CreateRemoteThread" => kernel32::CreateRemoteThread(emu),
        "CreateThread" => kernel32::CreateThread(emu),
        "CreateToolhelp32Snapshot" => kernel32::CreateToolhelp32Snapshot(emu),
        "ExitProcess" => kernel32::ExitProcess(emu),
        "ExpandEnvironmentStringsA" => kernel32::ExpandEnvironmentStringsA(emu),
        "ExpandEnvironmentStringsW" => kernel32::ExpandEnvironmentStringsW(emu),
        "FlsAlloc" => kernel32::FlsAlloc(emu),
        "FlsGetValue" => kernel32::FlsGetValue(emu),
        "FlsSetValue" => kernel32::FlsSetValue(emu),
        "GetThreadId" => kernel32::GetThreadId(emu),
        "GetCurrentProcessId" => kernel32::GetCurrentProcessId(emu),
        "GetCurrentThreadId" => kernel32::GetCurrentThreadId(emu),
        "FreeEnvironmentStringsA" => kernel32::FreeEnvironmentStringsA(emu),
        "FreeEnvironmentStringsW" => kernel32::FreeEnvironmentStringsW(emu),
        "GetEnvironmentStrings" => kernel32::GetEnvironmentStrings(emu),
        "GetEnvironmentStringsW" => kernel32::GetEnvironmentStringsW(emu),
        "GetProcessAffinityMask" => kernel32::GetProcessAffinityMask(emu),
        "GetStartupInfoA" => kernel32::GetStartupInfoA(emu),
        "GetStartupInfoW" => kernel32::GetStartupInfoW(emu),
        "GetThreadContext" => kernel32::GetThreadContext(emu),
        "GetThreadPreferredUILanguages" => kernel32::GetThreadPreferredUILanguages(emu),
        "GetThreadUILanguage" => kernel32::GetThreadUILanguage(emu),
        "IsProcessorFeaturePresent" => kernel32::IsProcessorFeaturePresent(emu),
        "OpenProcess" => kernel32::OpenProcess(emu),
        "OpenThread" => kernel32::OpenThread(emu),
        "ResumeThread" => kernel32::ResumeThread(emu),
        "SetThreadContext" => kernel32::SetThreadContext(emu),
        "SetThreadLocale" => kernel32::SetThreadLocale(emu),
        "TerminateProcess" => kernel32::TerminateProcess(emu),
        "Thread32First" => kernel32::Thread32First(emu),
        "Thread32Next" => kernel32::Thread32Next(emu),
        "TlsAlloc" => kernel32::TlsAlloc(emu),
        "TlsFree" => kernel32::TlsFree(emu),
        "TlsGetValue" => kernel32::TlsGetValue(emu),
        "TlsSetValue" => kernel32::TlsSetValue(emu),
        "WinExec" => kernel32::WinExec(emu),
        "BaseDestroyVDMEnvironment" => kernel32::BaseDestroyVDMEnvironment(emu),
        "BaseExitThreadPoolThread" => kernel32::BaseExitThreadPoolThread(emu),
        "BasepAllocateActivationContextActivationBlock" => {
            kernel32::BasepAllocateActivationContextActivationBlock(emu)
        }
        "BasepConstructSxsCreateProcessMessage" => {
            kernel32::BasepConstructSxsCreateProcessMessage(emu)
        }
        "BasepFreeActivationContextActivationBlock" => {
            kernel32::BasepFreeActivationContextActivationBlock(emu)
        }
        "BasepIsProcessAllowed" => kernel32::BasepIsProcessAllowed(emu),
        "BasepProcessInvalidImage" => kernel32::BasepProcessInvalidImage(emu),
        "BasepReleaseSxsCreateProcessUtilityStruct" => {
            kernel32::BasepReleaseSxsCreateProcessUtilityStruct(emu)
        }
        "BasepTpIoCallback" => kernel32::BasepTpIoCallback(emu),
        "BasepTpIoCleanupCallback" => kernel32::BasepTpIoCleanupCallback(emu),
        "BasepTpIoFinalizationCallback" => kernel32::BasepTpIoFinalizationCallback(emu),
        "DisableThreadProfiling" => kernel32::DisableThreadProfiling(emu),
        "EnableThreadProfiling" => kernel32::EnableThreadProfiling(emu),
        "FreeMemoryJobObject" => kernel32::FreeMemoryJobObject(emu),
        "GetActiveProcessorCount" => kernel32::GetActiveProcessorCount(emu),
        "GetActiveProcessorGroupCount" => kernel32::GetActiveProcessorGroupCount(emu),
        "GetExitCodeProcessImplementation" => kernel32::GetExitCodeProcessImplementation(emu),
        "GetFirmwareEnvironmentVariableA" => kernel32::GetFirmwareEnvironmentVariableA(emu),
        "GetFirmwareEnvironmentVariableW" => kernel32::GetFirmwareEnvironmentVariableW(emu),
        "GetMaximumProcessorCount" => kernel32::GetMaximumProcessorCount(emu),
        "GetMaximumProcessorGroupCount" => kernel32::GetMaximumProcessorGroupCount(emu),
        "GetNumaNodeProcessorMask" => kernel32::GetNumaNodeProcessorMask(emu),
        "GetNumaProcessorNode" => kernel32::GetNumaProcessorNode(emu),
        "GetNumaProcessorNodeEx" => kernel32::GetNumaProcessorNodeEx(emu),
        "GetProcessIoCounters" => kernel32::GetProcessIoCounters(emu),
        "GetThreadSelectorEntry" => kernel32::GetThreadSelectorEntry(emu),
        "IsThreadAFiber" => kernel32::IsThreadAFiber(emu),
        "NtVdm64CreateProcessInternalW" => kernel32::NtVdm64CreateProcessInternalW(emu),
        "NtWow64CsrBasepCreateProcess" => kernel32::NtWow64CsrBasepCreateProcess(emu),
        "NtWow64CsrBasepGetProcessShutdownParam" => {
            kernel32::NtWow64CsrBasepGetProcessShutdownParam(emu)
        }
        "NtWow64CsrBasepSetProcessShutdownParam" => {
            kernel32::NtWow64CsrBasepSetProcessShutdownParam(emu)
        }
        "OpenJobObjectA" => kernel32::OpenJobObjectA(emu),
        "OpenJobObjectW" => kernel32::OpenJobObjectW(emu),
        "Process32FirstW" => kernel32::Process32FirstW(emu),
        "Process32Next" => kernel32::Process32Next(emu),
        "Process32NextW" => kernel32::Process32NextW(emu),
        "QueryInformationJobObject" => kernel32::QueryInformationJobObject(emu),
        "QueryIoRateControlInformationJobObject" => {
            kernel32::QueryIoRateControlInformationJobObject(emu)
        }
        "QueryThreadProfiling" => kernel32::QueryThreadProfiling(emu),
        "ReadThreadProfilingData" => kernel32::ReadThreadProfilingData(emu),
        "SetEnvironmentStringsA" => kernel32::SetEnvironmentStringsA(emu),
        "SetFirmwareEnvironmentVariableA" => kernel32::SetFirmwareEnvironmentVariableA(emu),
        "SetFirmwareEnvironmentVariableW" => kernel32::SetFirmwareEnvironmentVariableW(emu),
        "SetInformationJobObject" => kernel32::SetInformationJobObject(emu),
        "SetIoRateControlInformationJobObject" => {
            kernel32::SetIoRateControlInformationJobObject(emu)
        }
        "SetProcessAffinityMask" => kernel32::SetProcessAffinityMask(emu),
        "SetThreadAffinityMask" => kernel32::SetThreadAffinityMask(emu),
        "SetThreadExecutionState" => kernel32::SetThreadExecutionState(emu),
        "Wow64EnableWow64FsRedirection" => kernel32::Wow64EnableWow64FsRedirection(emu),
        "Wow64GetThreadSelectorEntry" => kernel32::Wow64GetThreadSelectorEntry(emu),
        "Wow64SuspendThread" => kernel32::Wow64SuspendThread(emu),
        "Wow64SystemServiceCall" => kernel32::Wow64SystemServiceCall(emu),
        "Wow64TransitionResolve" => kernel32::Wow64TransitionResolve(emu),
        "AssignProcessToJobObject" => kernel32::AssignProcessToJobObject(emu),
        "BaseElevationPostProcessing" => kernel32::BaseElevationPostProcessing(emu),
        "BaseThreadInitThunk" => kernel32::BaseThreadInitThunk(emu),
        "CallbackMayRunLong" => kernel32::CallbackMayRunLong(emu),
        "CancelThreadpoolIo" => kernel32::CancelThreadpoolIo(emu),
        "CloseThreadpool" => kernel32::CloseThreadpool(emu),
        "CloseThreadpoolCleanupGroup" => kernel32::CloseThreadpoolCleanupGroup(emu),
        "CloseThreadpoolCleanupGroupMembers" => kernel32::CloseThreadpoolCleanupGroupMembers(emu),
        "CloseThreadpoolIo" => kernel32::CloseThreadpoolIo(emu),
        "CloseThreadpoolTimer" => kernel32::CloseThreadpoolTimer(emu),
        "CloseThreadpoolWait" => kernel32::CloseThreadpoolWait(emu),
        "CloseThreadpoolWork" => kernel32::CloseThreadpoolWork(emu),
        "ConvertFiberToThread" => kernel32::ConvertFiberToThread(emu),
        "ConvertThreadToFiber" => kernel32::ConvertThreadToFiber(emu),
        "ConvertThreadToFiberEx" => kernel32::ConvertThreadToFiberEx(emu),
        "CopyContext" => kernel32::CopyContext(emu),
        "CreateFiber" => kernel32::CreateFiber(emu),
        "CreateFiberEx" => kernel32::CreateFiberEx(emu),
        "CreateJobObjectA" => kernel32::CreateJobObjectA(emu),
        "CreateJobObjectW" => kernel32::CreateJobObjectW(emu),
        "CreateJobSet" => kernel32::CreateJobSet(emu),
        "CreateProcessAsUserA" => kernel32::CreateProcessAsUserA(emu),
        "CreateProcessAsUserW" => kernel32::CreateProcessAsUserW(emu),
        "CreateProcessInternalA" => kernel32::CreateProcessInternalA(emu),
        "CreateProcessInternalW" => kernel32::CreateProcessInternalW(emu),
        "CreateRemoteThreadEx" => kernel32::CreateRemoteThreadEx(emu),
        "CreateThreadpool" => kernel32::CreateThreadpool(emu),
        "CreateThreadpoolCleanupGroup" => kernel32::CreateThreadpoolCleanupGroup(emu),
        "CreateThreadpoolIo" => kernel32::CreateThreadpoolIo(emu),
        "CreateThreadpoolTimer" => kernel32::CreateThreadpoolTimer(emu),
        "CreateThreadpoolWait" => kernel32::CreateThreadpoolWait(emu),
        "CreateThreadpoolWork" => kernel32::CreateThreadpoolWork(emu),
        "DeleteFiber" => kernel32::DeleteFiber(emu),
        "DeleteProcThreadAttributeList" => kernel32::DeleteProcThreadAttributeList(emu),
        "DisassociateCurrentThreadFromCallback" => {
            kernel32::DisassociateCurrentThreadFromCallback(emu)
        }
        "EnableProcessOptionalXStateFeatures" => kernel32::EnableProcessOptionalXStateFeatures(emu),
        "ExitVDM" => kernel32::ExitVDM(emu),
        "FlsFree" => kernel32::FlsFree(emu),
        "FlsGetValue2" => kernel32::FlsGetValue2(emu),
        "FlushProcessWriteBuffers" => kernel32::FlushProcessWriteBuffers(emu),
        "GetCurrentProcessorNumber" => kernel32::GetCurrentProcessorNumber(emu),
        "GetCurrentProcessorNumberEx" => kernel32::GetCurrentProcessorNumberEx(emu),
        "GetCurrentThreadStackLimits" => kernel32::GetCurrentThreadStackLimits(emu),
        "GetEnvironmentStringsA" => kernel32::GetEnvironmentStringsA(emu),
        "GetEnvironmentVariableA" => kernel32::GetEnvironmentVariableA(emu),
        "GetExitCodeProcess" => kernel32::GetExitCodeProcess(emu),
        "GetExitCodeThread" => kernel32::GetExitCodeThread(emu),
        "GetFirmwareEnvironmentVariableExA" => kernel32::GetFirmwareEnvironmentVariableExA(emu),
        "GetFirmwareEnvironmentVariableExW" => kernel32::GetFirmwareEnvironmentVariableExW(emu),
        "GetLogicalProcessorInformation" => kernel32::GetLogicalProcessorInformation(emu),
        "GetLogicalProcessorInformationEx" => kernel32::GetLogicalProcessorInformationEx(emu),
        "GetNumaNodeProcessorMask2" => kernel32::GetNumaNodeProcessorMask2(emu),
        "GetNumaNodeProcessorMaskEx" => kernel32::GetNumaNodeProcessorMaskEx(emu),
        "GetPriorityClass" => kernel32::GetPriorityClass(emu),
        "GetProcessDefaultCpuSetMasks" => kernel32::GetProcessDefaultCpuSetMasks(emu),
        "GetProcessDefaultCpuSets" => kernel32::GetProcessDefaultCpuSets(emu),
        "GetProcessGroupAffinity" => kernel32::GetProcessGroupAffinity(emu),
        "GetProcessId" => kernel32::GetProcessId(emu),
        "GetProcessIdOfThread" => kernel32::GetProcessIdOfThread(emu),
        "GetProcessInformation" => kernel32::GetProcessInformation(emu),
        "GetProcessPreferredUILanguages" => kernel32::GetProcessPreferredUILanguages(emu),
        "GetProcessPriorityBoost" => kernel32::GetProcessPriorityBoost(emu),
        "GetProcessShutdownParameters" => kernel32::GetProcessShutdownParameters(emu),
        "GetProcessTimes" => kernel32::GetProcessTimes(emu),
        "GetProcessVersion" => kernel32::GetProcessVersion(emu),
        "GetProcessesInVirtualizationContext" => kernel32::GetProcessesInVirtualizationContext(emu),
        "GetProcessorSystemCycleTime" => kernel32::GetProcessorSystemCycleTime(emu),
        "GetThreadDescription" => kernel32::GetThreadDescription(emu),
        "GetThreadEnabledXStateFeatures" => kernel32::GetThreadEnabledXStateFeatures(emu),
        "GetThreadErrorMode" => kernel32::GetThreadErrorMode(emu),
        "GetThreadGroupAffinity" => kernel32::GetThreadGroupAffinity(emu),
        "GetThreadIOPendingFlag" => kernel32::GetThreadIOPendingFlag(emu),
        "GetThreadIdealProcessorEx" => kernel32::GetThreadIdealProcessorEx(emu),
        "GetThreadInformation" => kernel32::GetThreadInformation(emu),
        "GetThreadPriority" => kernel32::GetThreadPriority(emu),
        "GetThreadPriorityBoost" => kernel32::GetThreadPriorityBoost(emu),
        "GetThreadSelectedCpuSetMasks" => kernel32::GetThreadSelectedCpuSetMasks(emu),
        "GetThreadSelectedCpuSets" => kernel32::GetThreadSelectedCpuSets(emu),
        "GetThreadTimes" => kernel32::GetThreadTimes(emu),
        "InitializeContext" => kernel32::InitializeContext(emu),
        "InitializeContext2" => kernel32::InitializeContext2(emu),
        "InitializeProcThreadAttributeList" => kernel32::InitializeProcThreadAttributeList(emu),
        "IsProcessCritical" => kernel32::IsProcessCritical(emu),
        "IsProcessInJob" => kernel32::IsProcessInJob(emu),
        "IsThreadpoolTimerSet" => kernel32::IsThreadpoolTimerSet(emu),
        "IsUserCetAvailableInEnvironment" => kernel32::IsUserCetAvailableInEnvironment(emu),
        "IsWow64Process2" => kernel32::IsWow64Process2(emu),
        "K32EnumProcesses" => kernel32::K32EnumProcesses(emu),
        "K32GetProcessImageFileNameA" => kernel32::K32GetProcessImageFileNameA(emu),
        "K32GetProcessImageFileNameW" => kernel32::K32GetProcessImageFileNameW(emu),
        "K32InitializeProcessForWsWatch" => kernel32::K32InitializeProcessForWsWatch(emu),
        "ProcessIdToSessionId" => kernel32::ProcessIdToSessionId(emu),
        "PssCaptureSnapshot" => kernel32::PssCaptureSnapshot(emu),
        "PssDuplicateSnapshot" => kernel32::PssDuplicateSnapshot(emu),
        "PssFreeSnapshot" => kernel32::PssFreeSnapshot(emu),
        "PssQuerySnapshot" => kernel32::PssQuerySnapshot(emu),
        "PssWalkMarkerCreate" => kernel32::PssWalkMarkerCreate(emu),
        "PssWalkMarkerFree" => kernel32::PssWalkMarkerFree(emu),
        "PssWalkMarkerGetPosition" => kernel32::PssWalkMarkerGetPosition(emu),
        "PssWalkMarkerRewind" => kernel32::PssWalkMarkerRewind(emu),
        "PssWalkMarkerSeek" => kernel32::PssWalkMarkerSeek(emu),
        "PssWalkMarkerSeekToBeginning" => kernel32::PssWalkMarkerSeekToBeginning(emu),
        "PssWalkMarkerSetPosition" => kernel32::PssWalkMarkerSetPosition(emu),
        "PssWalkMarkerTell" => kernel32::PssWalkMarkerTell(emu),
        "PssWalkSnapshot" => kernel32::PssWalkSnapshot(emu),
        "QueryFullProcessImageNameA" => kernel32::QueryFullProcessImageNameA(emu),
        "QueryFullProcessImageNameW" => kernel32::QueryFullProcessImageNameW(emu),
        "QueryIdleProcessorCycleTime" => kernel32::QueryIdleProcessorCycleTime(emu),
        "QueryIdleProcessorCycleTimeEx" => kernel32::QueryIdleProcessorCycleTimeEx(emu),
        "QueryProcessAffinityUpdateMode" => kernel32::QueryProcessAffinityUpdateMode(emu),
        "QueryProcessCycleTime" => kernel32::QueryProcessCycleTime(emu),
        "QueryThreadCycleTime" => kernel32::QueryThreadCycleTime(emu),
        "QueryThreadpoolStackInformation" => kernel32::QueryThreadpoolStackInformation(emu),
        "QueueUserAPC" => kernel32::QueueUserAPC(emu),
        "QueueUserAPC2" => kernel32::QueueUserAPC2(emu),
        "QueueUserWorkItem" => kernel32::QueueUserWorkItem(emu),
        "RtlCaptureContext" => kernel32::RtlCaptureContext(emu),
        "SetEnvironmentStringsW" => kernel32::SetEnvironmentStringsW(emu),
        "SetFirmwareEnvironmentVariableExA" => kernel32::SetFirmwareEnvironmentVariableExA(emu),
        "SetFirmwareEnvironmentVariableExW" => kernel32::SetFirmwareEnvironmentVariableExW(emu),
        "SetPriorityClass" => kernel32::SetPriorityClass(emu),
        "SetProcessAffinityUpdateMode" => kernel32::SetProcessAffinityUpdateMode(emu),
        "SetProcessDefaultCpuSetMasks" => kernel32::SetProcessDefaultCpuSetMasks(emu),
        "SetProcessDefaultCpuSets" => kernel32::SetProcessDefaultCpuSets(emu),
        "SetProcessDynamicEHContinuationTargets" => {
            kernel32::SetProcessDynamicEHContinuationTargets(emu)
        }
        "SetProcessDynamicEnforcedCetCompatibleRanges" => {
            kernel32::SetProcessDynamicEnforcedCetCompatibleRanges(emu)
        }
        "SetProcessInformation" => kernel32::SetProcessInformation(emu),
        "SetProcessPreferredUILanguages" => kernel32::SetProcessPreferredUILanguages(emu),
        "SetProcessPriorityBoost" => kernel32::SetProcessPriorityBoost(emu),
        "SetProcessShutdownParameters" => kernel32::SetProcessShutdownParameters(emu),
        "SetThreadDescription" => kernel32::SetThreadDescription(emu),
        "SetThreadErrorMode" => kernel32::SetThreadErrorMode(emu),
        "SetThreadGroupAffinity" => kernel32::SetThreadGroupAffinity(emu),
        "SetThreadIdealProcessor" => kernel32::SetThreadIdealProcessor(emu),
        "SetThreadIdealProcessorEx" => kernel32::SetThreadIdealProcessorEx(emu),
        "SetThreadInformation" => kernel32::SetThreadInformation(emu),
        "SetThreadPreferredUILanguages" => kernel32::SetThreadPreferredUILanguages(emu),
        "SetThreadPriority" => kernel32::SetThreadPriority(emu),
        "SetThreadPriorityBoost" => kernel32::SetThreadPriorityBoost(emu),
        "SetThreadSelectedCpuSetMasks" => kernel32::SetThreadSelectedCpuSetMasks(emu),
        "SetThreadSelectedCpuSets" => kernel32::SetThreadSelectedCpuSets(emu),
        "SetThreadStackGuarantee" => kernel32::SetThreadStackGuarantee(emu),
        "SetThreadUILanguage" => kernel32::SetThreadUILanguage(emu),
        "SetThreadpoolStackInformation" => kernel32::SetThreadpoolStackInformation(emu),
        "SetThreadpoolThreadMaximum" => kernel32::SetThreadpoolThreadMaximum(emu),
        "SetThreadpoolThreadMinimum" => kernel32::SetThreadpoolThreadMinimum(emu),
        "SetThreadpoolTimer" => kernel32::SetThreadpoolTimer(emu),
        "SetThreadpoolTimerEx" => kernel32::SetThreadpoolTimerEx(emu),
        "SetThreadpoolWait" => kernel32::SetThreadpoolWait(emu),
        "SetThreadpoolWaitEx" => kernel32::SetThreadpoolWaitEx(emu),
        "StartThreadpoolIo" => kernel32::StartThreadpoolIo(emu),
        "SubmitThreadpoolWork" => kernel32::SubmitThreadpoolWork(emu),
        "SuspendThread" => kernel32::SuspendThread(emu),
        "SwitchToFiber" => kernel32::SwitchToFiber(emu),
        "SwitchToThread" => kernel32::SwitchToThread(emu),
        "TerminateJobObject" => kernel32::TerminateJobObject(emu),
        "TerminateThread" => kernel32::TerminateThread(emu),
        "TlsGetValue2" => kernel32::TlsGetValue2(emu),
        "TrySubmitThreadpoolCallback" => kernel32::TrySubmitThreadpoolCallback(emu),
        "UpdateProcThreadAttribute" => kernel32::UpdateProcThreadAttribute(emu),
        "Wow64DisableWow64FsRedirection" => kernel32::Wow64DisableWow64FsRedirection(emu),
        "Wow64GetThreadContext" => kernel32::Wow64GetThreadContext(emu),
        "Wow64RevertWow64FsRedirection" => kernel32::Wow64RevertWow64FsRedirection(emu),
        "Wow64SetThreadContext" => kernel32::Wow64SetThreadContext(emu),
        "Wow64Transition" => kernel32::Wow64Transition(emu),
        "CreateUmsThreadContext" => kernel32::CreateUmsThreadContext(emu),
        "DeleteUmsThreadContext" => kernel32::DeleteUmsThreadContext(emu),
        "EnterUmsSchedulingMode" => kernel32::EnterUmsSchedulingMode(emu),
        "ExecuteUmsThread" => kernel32::ExecuteUmsThread(emu),
        "GetCurrentUmsThread" => kernel32::GetCurrentUmsThread(emu),
        "GetNextUmsListItem" => kernel32::GetNextUmsListItem(emu),
        "GetUmsSystemThreadInformation" => kernel32::GetUmsSystemThreadInformation(emu),
        "QueryUmsThreadInformation" => kernel32::QueryUmsThreadInformation(emu),
        "RtlExitUserProcess" => kernel32::RtlExitUserProcess(emu),
        "RtlRestoreContext" => kernel32::RtlRestoreContext(emu),
        "SetUmsThreadInformation" => kernel32::SetUmsThreadInformation(emu),
        "UmsThreadYield" => kernel32::UmsThreadYield(emu),
        _ => return false,
    }
    true
}
