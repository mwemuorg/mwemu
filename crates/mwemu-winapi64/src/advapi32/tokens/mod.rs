use crate::emu::Emu;
use crate::winapi::winapi64::advapi32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "AdjustTokenPrivileges" => advapi32::AdjustTokenPrivileges(emu),
        "LookupPrivilegeValueW" => advapi32::LookupPrivilegeValueW(emu),
        "AccessCheck" => advapi32::AccessCheck(emu),
        "AccessCheckAndAuditAlarmA" => advapi32::AccessCheckAndAuditAlarmA(emu),
        "AccessCheckAndAuditAlarmW" => advapi32::AccessCheckAndAuditAlarmW(emu),
        "AccessCheckByType" => advapi32::AccessCheckByType(emu),
        "AccessCheckByTypeAndAuditAlarmA" => advapi32::AccessCheckByTypeAndAuditAlarmA(emu),
        "AccessCheckByTypeAndAuditAlarmW" => advapi32::AccessCheckByTypeAndAuditAlarmW(emu),
        "AccessCheckByTypeResultList" => advapi32::AccessCheckByTypeResultList(emu),
        "AccessCheckByTypeResultListAndAuditAlarmA" => {
            advapi32::AccessCheckByTypeResultListAndAuditAlarmA(emu)
        }
        "AccessCheckByTypeResultListAndAuditAlarmByHandleA" => {
            advapi32::AccessCheckByTypeResultListAndAuditAlarmByHandleA(emu)
        }
        "AccessCheckByTypeResultListAndAuditAlarmByHandleW" => {
            advapi32::AccessCheckByTypeResultListAndAuditAlarmByHandleW(emu)
        }
        "AccessCheckByTypeResultListAndAuditAlarmW" => {
            advapi32::AccessCheckByTypeResultListAndAuditAlarmW(emu)
        }
        "AdjustTokenGroups" => advapi32::AdjustTokenGroups(emu),
        "AuditComputeEffectivePolicyByToken" => advapi32::AuditComputeEffectivePolicyByToken(emu),
        "BuildImpersonateExplicitAccessWithNameA" => {
            advapi32::BuildImpersonateExplicitAccessWithNameA(emu)
        }
        "BuildImpersonateExplicitAccessWithNameW" => {
            advapi32::BuildImpersonateExplicitAccessWithNameW(emu)
        }
        "BuildImpersonateTrusteeA" => advapi32::BuildImpersonateTrusteeA(emu),
        "BuildImpersonateTrusteeW" => advapi32::BuildImpersonateTrusteeW(emu),
        "CheckTokenMembership" => advapi32::CheckTokenMembership(emu),
        "ComputeAccessTokenFromCodeAuthzLevel" => {
            advapi32::ComputeAccessTokenFromCodeAuthzLevel(emu)
        }
        "CreateProcessAsUserA" => advapi32::CreateProcessAsUserA(emu),
        "CreateProcessAsUserW" => advapi32::CreateProcessAsUserW(emu),
        "CreateProcessWithLogonW" => advapi32::CreateProcessWithLogonW(emu),
        "CreateProcessWithTokenW" => advapi32::CreateProcessWithTokenW(emu),
        "CreateRestrictedToken" => advapi32::CreateRestrictedToken(emu),
        "CredReadByTokenHandle" => advapi32::CredReadByTokenHandle(emu),
        "DuplicateToken" => advapi32::DuplicateToken(emu),
        "DuplicateTokenEx" => advapi32::DuplicateTokenEx(emu),
        "GetTokenInformation" => advapi32::GetTokenInformation(emu),
        "ImpersonateAnonymousToken" => advapi32::ImpersonateAnonymousToken(emu),
        "ImpersonateLoggedOnUser" => advapi32::ImpersonateLoggedOnUser(emu),
        "ImpersonateNamedPipeClient" => advapi32::ImpersonateNamedPipeClient(emu),
        "ImpersonateSelf" => advapi32::ImpersonateSelf(emu),
        "IsTokenRestricted" => advapi32::IsTokenRestricted(emu),
        "IsTokenUntrusted" => advapi32::IsTokenUntrusted(emu),
        "LogonSecondaryUserIntoSessionW" => advapi32::LogonSecondaryUserIntoSessionW(emu),
        "LogonUserA" => advapi32::LogonUserA(emu),
        "LogonUserExA" => advapi32::LogonUserExA(emu),
        "LogonUserExExW" => advapi32::LogonUserExExW(emu),
        "LogonUserExW" => advapi32::LogonUserExW(emu),
        "LogonUserW" => advapi32::LogonUserW(emu),
        "LookupPrivilegeDisplayNameA" => advapi32::LookupPrivilegeDisplayNameA(emu),
        "LookupPrivilegeDisplayNameW" => advapi32::LookupPrivilegeDisplayNameW(emu),
        "LookupPrivilegeNameA" => advapi32::LookupPrivilegeNameA(emu),
        "LookupPrivilegeNameW" => advapi32::LookupPrivilegeNameW(emu),
        "LookupPrivilegeValueA" => advapi32::LookupPrivilegeValueA(emu),
        "LsaAddPrivilegesToAccount" => advapi32::LsaAddPrivilegesToAccount(emu),
        "LsaEnumeratePrivileges" => advapi32::LsaEnumeratePrivileges(emu),
        "LsaEnumeratePrivilegesOfAccount" => advapi32::LsaEnumeratePrivilegesOfAccount(emu),
        "LsaLookupPrivilegeDisplayName" => advapi32::LsaLookupPrivilegeDisplayName(emu),
        "LsaLookupPrivilegeName" => advapi32::LsaLookupPrivilegeName(emu),
        "LsaLookupPrivilegeValue" => advapi32::LsaLookupPrivilegeValue(emu),
        "LsaRemovePrivilegesFromAccount" => advapi32::LsaRemovePrivilegesFromAccount(emu),
        "ObjectPrivilegeAuditAlarmA" => advapi32::ObjectPrivilegeAuditAlarmA(emu),
        "ObjectPrivilegeAuditAlarmW" => advapi32::ObjectPrivilegeAuditAlarmW(emu),
        "OpenProcessToken" => advapi32::OpenProcessToken(emu),
        "OpenThreadToken" => advapi32::OpenThreadToken(emu),
        "OpenThreadWaitChainSession" => advapi32::OpenThreadWaitChainSession(emu),
        "PrivilegeCheck" => advapi32::PrivilegeCheck(emu),
        "RevertToSelf" => advapi32::RevertToSelf(emu),
        "SaferComputeTokenFromLevel" => advapi32::SaferComputeTokenFromLevel(emu),
        "SaferiCompareTokenLevels" => advapi32::SaferiCompareTokenLevels(emu),
        "SetThreadToken" => advapi32::SetThreadToken(emu),
        "SetTokenInformation" => advapi32::SetTokenInformation(emu),
        _ => return false,
    }
    true
}
