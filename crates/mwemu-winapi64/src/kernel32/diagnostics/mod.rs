use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "AddVectoredExceptionHandler" => kernel32::AddVectoredExceptionHandler(emu),
        "EtwEventEnabled" | "EventEnabled" => kernel32::EtwEventEnabled(emu),
        "EtwEventRegister" | "EventRegister" => kernel32::EtwEventRegister(emu),
        "EtwEventSetInformation" | "EventSetInformation" => kernel32::EtwEventSetInformation(emu),
        "EtwEventWriteTransfer" | "EventWriteTransfer" => kernel32::EtwEventWriteTransfer(emu),
        "CheckRemoteDebuggerPresent" => kernel32::CheckRemoteDebuggerPresent(emu),
        "IsDebuggerPresent" => kernel32::IsDebuggerPresent(emu),
        "OutputDebugStringA" => kernel32::OutputDebugStringA(emu),
        "OutputDebugStringW" => kernel32::OutputDebugStringW(emu),
        "RaiseException" => kernel32::RaiseException(emu),
        "SetUnhandledExceptionFilter" => kernel32::SetUnhandledExceptionFilter(emu),
        "UnhandledExceptionFilter" => kernel32::UnhandledExceptionFilter(emu),
        _ => return false,
    }
    true
}
