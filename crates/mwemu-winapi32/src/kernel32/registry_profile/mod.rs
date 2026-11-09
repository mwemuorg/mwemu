use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

#[path = "../reg_open_key_ex.rs"]
mod reg_open_key_ex;
pub use reg_open_key_ex::*;

#[path = "../write_profile_section.rs"]
mod write_profile_section;
pub use write_profile_section::*;

#[path = "../write_private_profile_struct.rs"]
mod write_private_profile_struct;
pub use write_private_profile_struct::*;

#[path = "../write_private_profile_section.rs"]
mod write_private_profile_section;
pub use write_private_profile_section::*;

#[path = "../reg_set_value_ex.rs"]
mod reg_set_value_ex;
pub use reg_set_value_ex::*;

#[path = "../reg_open_key.rs"]
mod reg_open_key;
pub use reg_open_key::*;

#[path = "../reg_create_key_ex.rs"]
mod reg_create_key_ex;
pub use reg_create_key_ex::*;

#[path = "../get_profile_section.rs"]
mod get_profile_section;
pub use get_profile_section::*;

#[path = "../get_profile_int.rs"]
mod get_profile_int;
pub use get_profile_int::*;

#[path = "../get_private_profile_struct.rs"]
mod get_private_profile_struct;
pub use get_private_profile_struct::*;

#[path = "../get_private_profile_section_names.rs"]
mod get_private_profile_section_names;
pub use get_private_profile_section_names::*;

#[path = "../get_private_profile_section.rs"]
mod get_private_profile_section;
pub use get_private_profile_section::*;

#[path = "../get_private_profile_int.rs"]
mod get_private_profile_int;
pub use get_private_profile_int::*;

#[path = "../reg_close_key.rs"]
mod reg_close_key;
pub use reg_close_key::*;

#[path = "../termsrv_convert_sys_root_to_user_dir.rs"]
mod termsrv_convert_sys_root_to_user_dir;
pub use termsrv_convert_sys_root_to_user_dir::*;

#[path = "../termsrv_create_reg_entry.rs"]
mod termsrv_create_reg_entry;
pub use termsrv_create_reg_entry::*;

#[path = "../termsrv_delete_key.rs"]
mod termsrv_delete_key;
pub use termsrv_delete_key::*;

#[path = "../termsrv_delete_value.rs"]
mod termsrv_delete_value;
pub use termsrv_delete_value::*;

#[path = "../termsrv_get_pre_set_value.rs"]
mod termsrv_get_pre_set_value;
pub use termsrv_get_pre_set_value::*;

#[path = "../termsrv_open_reg_entry.rs"]
mod termsrv_open_reg_entry;
pub use termsrv_open_reg_entry::*;

#[path = "../termsrv_open_user_classes.rs"]
mod termsrv_open_user_classes;
pub use termsrv_open_user_classes::*;

#[path = "../termsrv_restore_key.rs"]
mod termsrv_restore_key;
pub use termsrv_restore_key::*;

#[path = "../termsrv_set_value_key.rs"]
mod termsrv_set_value_key;
pub use termsrv_set_value_key::*;

#[path = "../termsrv_sync_user_ini_file.rs"]
mod termsrv_sync_user_ini_file;
pub use termsrv_sync_user_ini_file::*;

#[path = "../termsrv_sync_user_ini_file_ext.rs"]
mod termsrv_sync_user_ini_file_ext;
pub use termsrv_sync_user_ini_file_ext::*;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "RegCloseKey" => kernel32::RegCloseKey(emu),
        "RegCreateKeyExA" => kernel32::RegCreateKeyExA(emu),
        "RegCreateKeyExW" => kernel32::RegCreateKeyExW(emu),
        "RegOpenKeyA" => kernel32::RegOpenKeyA(emu),
        "RegOpenKeyExW" => kernel32::RegOpenKeyExW(emu),
        "RegOpenKeyW" => kernel32::RegOpenKeyW(emu),
        "RegSetValueExA" => kernel32::RegSetValueExA(emu),
        "RegSetValueExW" => kernel32::RegSetValueExW(emu),
        "GetPrivateProfileIntA" => kernel32::GetPrivateProfileIntA(emu),
        "GetPrivateProfileIntW" => kernel32::GetPrivateProfileIntW(emu),
        "GetPrivateProfileSectionA" => kernel32::GetPrivateProfileSectionA(emu),
        "GetPrivateProfileSectionNamesA" => kernel32::GetPrivateProfileSectionNamesA(emu),
        "GetPrivateProfileSectionNamesW" => kernel32::GetPrivateProfileSectionNamesW(emu),
        "GetPrivateProfileSectionW" => kernel32::GetPrivateProfileSectionW(emu),
        "GetPrivateProfileStringA" => kernel32::GetPrivateProfileStringA(emu),
        "GetPrivateProfileStringW" => kernel32::GetPrivateProfileStringW(emu),
        "GetPrivateProfileStructA" => kernel32::GetPrivateProfileStructA(emu),
        "GetPrivateProfileStructW" => kernel32::GetPrivateProfileStructW(emu),
        "GetProfileIntA" => kernel32::GetProfileIntA(emu),
        "GetProfileIntW" => kernel32::GetProfileIntW(emu),
        "GetProfileSectionA" => kernel32::GetProfileSectionA(emu),
        "GetProfileSectionW" => kernel32::GetProfileSectionW(emu),
        "GetProfileStringA" => kernel32::GetProfileStringA(emu),
        "GetProfileStringW" => kernel32::GetProfileStringW(emu),
        "GetSystemRegistryQuota" => kernel32::GetSystemRegistryQuota(emu),
        "RegisterApplicationRecoveryCallback" => kernel32::RegisterApplicationRecoveryCallback(emu),
        "RegisterApplicationRestart" => kernel32::RegisterApplicationRestart(emu),
        "RegisterWaitForInputIdle" => kernel32::RegisterWaitForInputIdle(emu),
        "RegisterWaitUntilOOBECompleted" => kernel32::RegisterWaitUntilOOBECompleted(emu),
        "SdbpCheckMatchingRegistry" => kernel32::SdbpCheckMatchingRegistry(emu),
        "SdbpCheckMatchingWildcardRegistry" => kernel32::SdbpCheckMatchingWildcardRegistry(emu),
        "TermsrvConvertSysRootToUserDir" => kernel32::TermsrvConvertSysRootToUserDir(emu),
        "TermsrvCreateRegEntry" => kernel32::TermsrvCreateRegEntry(emu),
        "TermsrvDeleteKey" => kernel32::TermsrvDeleteKey(emu),
        "TermsrvDeleteValue" => kernel32::TermsrvDeleteValue(emu),
        "TermsrvGetPreSetValue" => kernel32::TermsrvGetPreSetValue(emu),
        "TermsrvOpenRegEntry" => kernel32::TermsrvOpenRegEntry(emu),
        "TermsrvOpenUserClasses" => kernel32::TermsrvOpenUserClasses(emu),
        "TermsrvRestoreKey" => kernel32::TermsrvRestoreKey(emu),
        "TermsrvSetValueKey" => kernel32::TermsrvSetValueKey(emu),
        "TermsrvSyncUserIniFile" => kernel32::TermsrvSyncUserIniFile(emu),
        "TermsrvSyncUserIniFileExt" => kernel32::TermsrvSyncUserIniFileExt(emu),
        "WritePrivateProfileSectionA" => kernel32::WritePrivateProfileSectionA(emu),
        "WritePrivateProfileSectionW" => kernel32::WritePrivateProfileSectionW(emu),
        "WritePrivateProfileStringA" => kernel32::WritePrivateProfileStringA(emu),
        "WritePrivateProfileStringW" => kernel32::WritePrivateProfileStringW(emu),
        "WritePrivateProfileStructA" => kernel32::WritePrivateProfileStructA(emu),
        "WritePrivateProfileStructW" => kernel32::WritePrivateProfileStructW(emu),
        "WriteProfileSectionA" => kernel32::WriteProfileSectionA(emu),
        "WriteProfileSectionW" => kernel32::WriteProfileSectionW(emu),
        "WriteProfileStringA" => kernel32::WriteProfileStringA(emu),
        "WriteProfileStringW" => kernel32::WriteProfileStringW(emu),
        "RegCopyTreeW" => kernel32::RegCopyTreeW(emu),
        "RegDeleteKeyExA" => kernel32::RegDeleteKeyExA(emu),
        "RegDeleteKeyExW" => kernel32::RegDeleteKeyExW(emu),
        "RegDeleteTreeA" => kernel32::RegDeleteTreeA(emu),
        "RegDeleteTreeW" => kernel32::RegDeleteTreeW(emu),
        "RegDeleteValueA" => kernel32::RegDeleteValueA(emu),
        "RegDeleteValueW" => kernel32::RegDeleteValueW(emu),
        "RegDisablePredefinedCacheEx" => kernel32::RegDisablePredefinedCacheEx(emu),
        "RegEnumKeyExA" => kernel32::RegEnumKeyExA(emu),
        "RegEnumKeyExW" => kernel32::RegEnumKeyExW(emu),
        "RegEnumValueA" => kernel32::RegEnumValueA(emu),
        "RegEnumValueW" => kernel32::RegEnumValueW(emu),
        "RegFlushKey" => kernel32::RegFlushKey(emu),
        "RegGetKeySecurity" => kernel32::RegGetKeySecurity(emu),
        "RegGetValueA" => kernel32::RegGetValueA(emu),
        "RegGetValueW" => kernel32::RegGetValueW(emu),
        "RegLoadKeyA" => kernel32::RegLoadKeyA(emu),
        "RegLoadKeyW" => kernel32::RegLoadKeyW(emu),
        "RegLoadMUIStringA" => kernel32::RegLoadMUIStringA(emu),
        "RegLoadMUIStringW" => kernel32::RegLoadMUIStringW(emu),
        "RegNotifyChangeKeyValue" => kernel32::RegNotifyChangeKeyValue(emu),
        "RegOpenCurrentUser" => kernel32::RegOpenCurrentUser(emu),
        "RegOpenKeyExA" => kernel32::RegOpenKeyExA(emu),
        "RegOpenUserClassesRoot" => kernel32::RegOpenUserClassesRoot(emu),
        "RegQueryInfoKeyA" => kernel32::RegQueryInfoKeyA(emu),
        "RegQueryInfoKeyW" => kernel32::RegQueryInfoKeyW(emu),
        "RegQueryValueExA" => kernel32::RegQueryValueExA(emu),
        "RegQueryValueExW" => kernel32::RegQueryValueExW(emu),
        "RegRestoreKeyA" => kernel32::RegRestoreKeyA(emu),
        "RegRestoreKeyW" => kernel32::RegRestoreKeyW(emu),
        "RegSaveKeyExA" => kernel32::RegSaveKeyExA(emu),
        "RegSaveKeyExW" => kernel32::RegSaveKeyExW(emu),
        "RegSetKeySecurity" => kernel32::RegSetKeySecurity(emu),
        "RegUnLoadKeyA" => kernel32::RegUnLoadKeyA(emu),
        "RegUnLoadKeyW" => kernel32::RegUnLoadKeyW(emu),
        "RegisterBadMemoryNotification" => kernel32::RegisterBadMemoryNotification(emu),
        "RegisterWaitForSingleObject" => kernel32::RegisterWaitForSingleObject(emu),
        "RegisterWaitForSingleObjectEx" => kernel32::RegisterWaitForSingleObjectEx(emu),
        "RegisterWowExec" => kernel32::RegisterWowExec(emu),
        "TermsrvAppInstallMode" => kernel32::TermsrvAppInstallMode(emu),
        "TermsrvGetWindowsDirectoryA" => kernel32::TermsrvGetWindowsDirectoryA(emu),
        "TermsrvGetWindowsDirectoryW" => kernel32::TermsrvGetWindowsDirectoryW(emu),
        "TermsrvSetKeySecurity" => kernel32::TermsrvSetKeySecurity(emu),
        _ => return false,
    }
    true
}
