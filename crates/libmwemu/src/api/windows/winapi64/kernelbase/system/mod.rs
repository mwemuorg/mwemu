use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "AreAllAccessesGranted" => kernelbase::AreAllAccessesGranted(emu),
        "AreAnyAccessesGranted" => kernelbase::AreAnyAccessesGranted(emu),
        "Beep" => kernelbase::Beep(emu),
        "CeipIsOptedIn" => kernelbase::CeipIsOptedIn(emu),
        "CtrlRoutine" => kernelbase::CtrlRoutine(emu),
        "FormatMessageA" => kernelbase::FormatMessageA(emu),
        "FormatMessageW" => kernelbase::FormatMessageW(emu),
        "GetLastError" => kernelbase::GetLastError(emu),
        "GetMachineTypeAttributes" => kernelbase::GetMachineTypeAttributes(emu),
        "GetNativeSystemInfo" => kernelbase::GetNativeSystemInfo(emu),
        "GetSystemAppDataKey" => kernelbase::GetSystemAppDataKey(emu),
        "GetSystemCpuSetInformation" => kernelbase::GetSystemCpuSetInformation(emu),
        "GetSystemFirmwareTable" => kernelbase::GetSystemFirmwareTable(emu),
        "GetSystemInfo" => kernelbase::GetSystemInfo(emu),
        "GetVersion" => kernelbase::GetVersion(emu),
        "GetVersionExA" => kernelbase::GetVersionExA(emu),
        "GetVersionExW" => kernelbase::GetVersionExW(emu),
        "InstallELAMCertificateInfo" => kernelbase::InstallELAMCertificateInfo(emu),
        "IsWow64GuestMachineSupported" => kernelbase::IsWow64GuestMachineSupported(emu),
        "MulDiv" => kernelbase::MulDiv(emu),
        "RestoreLastError" => kernelbase::RestoreLastError(emu),
        "SetLastError" => kernelbase::SetLastError(emu),
        "VerSetConditionMask" => kernelbase::VerSetConditionMask(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
