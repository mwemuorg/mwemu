use crate::emu::Emu;
use crate::winapi::winapi64::advapi32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "AbortSystemShutdownA" => advapi32::AbortSystemShutdownA(emu),
        "AbortSystemShutdownW" => advapi32::AbortSystemShutdownW(emu),
        "CheckForHiberboot" => advapi32::CheckForHiberboot(emu),
        "CloseThreadWaitChainSession" => advapi32::CloseThreadWaitChainSession(emu),
        "CveEventWrite" => advapi32::CveEventWrite(emu),
        "EnumDynamicTimeZoneInformation" => advapi32::EnumDynamicTimeZoneInformation(emu),
        "GetDynamicTimeZoneInformationEffectiveYears" => {
            advapi32::GetDynamicTimeZoneInformationEffectiveYears(emu)
        }
        "GetLocalManagedApplicationData" => advapi32::GetLocalManagedApplicationData(emu),
        "GetLocalManagedApplications" => advapi32::GetLocalManagedApplications(emu),
        "GetManagedApplicationCategories" => advapi32::GetManagedApplicationCategories(emu),
        "GetManagedApplications" => advapi32::GetManagedApplications(emu),
        "GetStringConditionFromBinary" => advapi32::GetStringConditionFromBinary(emu),
        "GetThreadWaitChain" => advapi32::GetThreadWaitChain(emu),
        "I_QueryTagInformation" => advapi32::I_QueryTagInformation(emu),
        "InitiateShutdownA" => advapi32::InitiateShutdownA(emu),
        "InitiateShutdownW" => advapi32::InitiateShutdownW(emu),
        "InitiateSystemShutdownA" => advapi32::InitiateSystemShutdownA(emu),
        "InitiateSystemShutdownExA" => advapi32::InitiateSystemShutdownExA(emu),
        "InitiateSystemShutdownExW" => advapi32::InitiateSystemShutdownExW(emu),
        "InitiateSystemShutdownW" => advapi32::InitiateSystemShutdownW(emu),
        "InstallApplication" => advapi32::InstallApplication(emu),
        "IsTextUnicode" => advapi32::IsTextUnicode(emu),
        "MIDL_user_free_Ext" => advapi32::MIDL_user_free_Ext(emu),
        "NotifyBootConfigStatus" => advapi32::NotifyBootConfigStatus(emu),
        "OperationEnd" => advapi32::OperationEnd(emu),
        "OperationStart" => advapi32::OperationStart(emu),
        "ProcessIdleTasks" => advapi32::ProcessIdleTasks(emu),
        "ProcessIdleTasksW" => advapi32::ProcessIdleTasksW(emu),
        "SetEntriesInAuditListA" => advapi32::SetEntriesInAuditListA(emu),
        "SetEntriesInAuditListW" => advapi32::SetEntriesInAuditListW(emu),
        "UninstallApplication" => advapi32::UninstallApplication(emu),
        "UnregisterIdleTask" => advapi32::UnregisterIdleTask(emu),
        "RegisterIdleTask" => advapi32::RegisterIdleTask(emu),
        "RegisterWaitChainCOMCallback" => advapi32::RegisterWaitChainCOMCallback(emu),
        _ => return false,
    }
    true
}
