use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "SetUnhandledExceptionFilter" => kernelbase::SetUnhandledExceptionFilter(emu),
        "EventActivityIdControl" => kernelbase::EventActivityIdControl(emu),
        "EventProviderEnabled" => kernelbase::EventProviderEnabled(emu),
        "EventUnregister" => kernelbase::EventUnregister(emu),
        "EventWrite" => kernelbase::EventWrite(emu),
        "EventWriteEx" => kernelbase::EventWriteEx(emu),
        "EventWriteString" => kernelbase::EventWriteString(emu),
        "GetTraceEnableFlags" => kernelbase::GetTraceEnableFlags(emu),
        "GetTraceEnableLevel" => kernelbase::GetTraceEnableLevel(emu),
        "GetTraceLoggerHandle" => kernelbase::GetTraceLoggerHandle(emu),
        "RegisterTraceGuidsW" => kernelbase::RegisterTraceGuidsW(emu),
        "TraceEvent" => kernelbase::TraceEvent(emu),
        "TraceMessage" => kernelbase::TraceMessage(emu),
        "TraceMessageVa" => kernelbase::TraceMessageVa(emu),
        "UnregisterTraceGuids" => kernelbase::UnregisterTraceGuids(emu),
        "AddVectoredContinueHandler" => kernelbase::AddVectoredContinueHandler(emu),
        "AddVectoredExceptionHandler" => kernelbase::AddVectoredExceptionHandler(emu),
        "CheckRemoteDebuggerPresent" => kernelbase::CheckRemoteDebuggerPresent(emu),
        "EventEnabled" => kernelbase::EventEnabled(emu),
        "EventRegister" => kernelbase::EventRegister(emu),
        "EventSetInformation" => kernelbase::EventSetInformation(emu),
        "EventWriteTransfer" => kernelbase::EventWriteTransfer(emu),
        "IsDebuggerPresent" => kernelbase::IsDebuggerPresent(emu),
        "OutputDebugStringA" => kernelbase::OutputDebugStringA(emu),
        "OutputDebugStringW" => kernelbase::OutputDebugStringW(emu),
        "RaiseException" => kernelbase::RaiseException(emu),
        "RaiseFailFastException" => kernelbase::RaiseFailFastException(emu),
        "RemoveVectoredContinueHandler" => kernelbase::RemoveVectoredContinueHandler(emu),
        "RemoveVectoredExceptionHandler" => kernelbase::RemoveVectoredExceptionHandler(emu),
        "UnhandledExceptionFilter" => kernelbase::UnhandledExceptionFilter(emu),
        "WaitForDebugEventEx" => kernelbase::WaitForDebugEventEx(emu),
        "WerSetFlags" => kernelbase::WerSetFlags(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
