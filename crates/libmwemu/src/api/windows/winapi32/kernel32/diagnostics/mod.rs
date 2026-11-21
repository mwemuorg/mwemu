use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "AddVectoredExceptionHandler" => kernel32::AddVectoredExceptionHandler(emu),
        "OutputDebugStringA" => kernel32::OutputDebugStringA(emu),
        "OutputDebugStringW" => kernel32::OutputDebugStringW(emu),
        "IsDebuggerPresent" => kernel32::IsDebuggerPresent(emu),
        "RaiseException" => kernel32::RaiseException(emu),
        "SetUnhandledExceptionFilter" => kernel32::SetUnhandledExceptionFilter(emu),
        "UnhandledExceptionFilter" => kernel32::UnhandledExceptionFilter(emu),
        "DebugSetProcessKillOnExit" => kernel32::DebugSetProcessKillOnExit(emu),
        "FatalExit" => kernel32::FatalExit(emu),
        "RaiseInvalid16BitExeError" => kernel32::RaiseInvalid16BitExeError(emu),
        "SbpIsTraceEnabled" => kernel32::SbpIsTraceEnabled(emu),
        "TraceLoggingRegisterEx_EventRegister_EventSetInformation" => {
            kernel32::TraceLoggingRegisterEx_EventRegister_EventSetInformation(emu)
        }
        "WerGetFlagsWorker" => kernel32::WerGetFlagsWorker(emu),
        "WerRegisterFileWorker" => kernel32::WerRegisterFileWorker(emu),
        "WerSetFlags" => kernel32::WerSetFlags(emu),
        "WerUnregisterFileWorker" => kernel32::WerUnregisterFileWorker(emu),
        "WerUnregisterMemoryBlockWorker" => kernel32::WerUnregisterMemoryBlockWorker(emu),
        "WerpAcquirePebLock" => kernel32::WerpAcquirePebLock(emu),
        "WerpCheckOkToRegister" => kernel32::WerpCheckOkToRegister(emu),
        "WerpGetDebugger" => kernel32::WerpGetDebugger(emu),
        "WerpInitPEBStore" => kernel32::WerpInitPEBStore(emu),
        "WerpInitiateRemoteRecovery" => kernel32::WerpInitiateRemoteRecovery(emu),
        "WerpLaunchAeDebug" => kernel32::WerpLaunchAeDebug(emu),
        "WerpMapProtectionLevel" => kernel32::WerpMapProtectionLevel(emu),
        "WerpNotifyUseStringResourceWorker" => kernel32::WerpNotifyUseStringResourceWorker(emu),
        "WerpRecoveryInvokedRemotely" => kernel32::WerpRecoveryInvokedRemotely(emu),
        "AddVectoredContinueHandler" => kernel32::AddVectoredContinueHandler(emu),
        "CheckRemoteDebuggerPresent" => kernel32::CheckRemoteDebuggerPresent(emu),
        "ContinueDebugEvent" => kernel32::ContinueDebugEvent(emu),
        "DebugActiveProcess" => kernel32::DebugActiveProcess(emu),
        "DebugActiveProcessStop" => kernel32::DebugActiveProcessStop(emu),
        "DebugBreak" => kernel32::DebugBreak(emu),
        "DebugBreakProcess" => kernel32::DebugBreakProcess(emu),
        "FatalAppExitA" => kernel32::FatalAppExitA(emu),
        "FatalAppExitW" => kernel32::FatalAppExitW(emu),
        "RaiseFailFastException" => kernel32::RaiseFailFastException(emu),
        "RemoveVectoredContinueHandler" => kernel32::RemoveVectoredContinueHandler(emu),
        "RemoveVectoredExceptionHandler" => kernel32::RemoveVectoredExceptionHandler(emu),
        "RtlCaptureStackBackTrace" => kernel32::RtlCaptureStackBackTrace(emu),
        "RtlUnwind" => kernel32::RtlUnwind(emu),
        "WaitForDebugEvent" => kernel32::WaitForDebugEvent(emu),
        "WaitForDebugEventEx" => kernel32::WaitForDebugEventEx(emu),
        "WerGetFlags" => kernel32::WerGetFlags(emu),
        "WerRegisterAdditionalProcess" => kernel32::WerRegisterAdditionalProcess(emu),
        "WerRegisterAppLocalDump" => kernel32::WerRegisterAppLocalDump(emu),
        "WerRegisterCustomMetadata" => kernel32::WerRegisterCustomMetadata(emu),
        "WerRegisterExcludedMemoryBlock" => kernel32::WerRegisterExcludedMemoryBlock(emu),
        "WerRegisterFile" => kernel32::WerRegisterFile(emu),
        "WerRegisterMemoryBlock" => kernel32::WerRegisterMemoryBlock(emu),
        "WerRegisterMemoryBlockWorker" => kernel32::WerRegisterMemoryBlockWorker(emu),
        "WerRegisterRuntimeExceptionModule" => kernel32::WerRegisterRuntimeExceptionModule(emu),
        "WerRegisterRuntimeExceptionModuleWorker" => {
            kernel32::WerRegisterRuntimeExceptionModuleWorker(emu)
        }
        "WerSetFlagsWorker" => kernel32::WerSetFlagsWorker(emu),
        "WerUnregisterAdditionalProcess" => kernel32::WerUnregisterAdditionalProcess(emu),
        "WerUnregisterAppLocalDump" => kernel32::WerUnregisterAppLocalDump(emu),
        "WerUnregisterCustomMetadata" => kernel32::WerUnregisterCustomMetadata(emu),
        "WerUnregisterExcludedMemoryBlock" => kernel32::WerUnregisterExcludedMemoryBlock(emu),
        "WerUnregisterFile" => kernel32::WerUnregisterFile(emu),
        "WerUnregisterMemoryBlock" => kernel32::WerUnregisterMemoryBlock(emu),
        "WerUnregisterRuntimeExceptionModule" => kernel32::WerUnregisterRuntimeExceptionModule(emu),
        "WerUnregisterRuntimeExceptionModuleWorker" => {
            kernel32::WerUnregisterRuntimeExceptionModuleWorker(emu)
        }
        "WerpNotifyLoadStringResourceWorker" => kernel32::WerpNotifyLoadStringResourceWorker(emu),
        "EtwEventEnabled" | "EventEnabled" => kernel32::EtwEventEnabled(emu),
        "EtwEventRegister" | "EventRegister" => kernel32::EtwEventRegister(emu),
        "EtwEventSetInformation" | "EventSetInformation" => kernel32::EtwEventSetInformation(emu),
        "EtwEventWriteTransfer" | "EventWriteTransfer" => kernel32::EtwEventWriteTransfer(emu),
        "RtlRaiseException" => kernel32::RtlRaiseException(emu),
        "RtlUnwindEx" => kernel32::RtlUnwindEx(emu),
        "RtlVirtualUnwind" => kernel32::RtlVirtualUnwind(emu),
        "RtlVirtualUnwind2" => kernel32::RtlVirtualUnwind2(emu),
        "K32GetPerformanceInfo" => kernel32::K32GetPerformanceInfo(emu),
        _ => return false,
    }
    true
}
