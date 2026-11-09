use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

#[path = "../set_locale_info.rs"]
mod set_locale_info;
pub use set_locale_info::*;

#[path = "../set_calendar_info.rs"]
mod set_calendar_info;
pub use set_calendar_info::*;

#[path = "../get_environment_variable.rs"]
mod get_environment_variable;
pub use get_environment_variable::*;

#[path = "../get_calendar_info.rs"]
mod get_calendar_info;
pub use get_calendar_info::*;

#[path = "../enum_ui_languages.rs"]
mod enum_ui_languages;
pub use enum_ui_languages::*;

#[path = "../enum_time_formats.rs"]
mod enum_time_formats;
pub use enum_time_formats::*;

#[path = "../enum_system_language_groups.rs"]
mod enum_system_language_groups;
pub use enum_system_language_groups::*;

#[path = "../enum_system_code_pages.rs"]
mod enum_system_code_pages;
pub use enum_system_code_pages::*;

#[path = "../enum_language_group_locales.rs"]
mod enum_language_group_locales;
pub use enum_language_group_locales::*;

#[path = "../enum_date_formats_ex.rs"]
mod enum_date_formats_ex;
pub use enum_date_formats_ex::*;

#[path = "../enum_date_formats.rs"]
mod enum_date_formats;
pub use enum_date_formats::*;

#[path = "../enum_calendar_info_ex.rs"]
mod enum_calendar_info_ex;
pub use enum_calendar_info_ex::*;

#[path = "../enum_calendar_info.rs"]
mod enum_calendar_info;
pub use enum_calendar_info::*;

#[path = "../verify_version_info.rs"]
mod verify_version_info;
pub use verify_version_info::*;

#[path = "../set_firmware_environment_variable.rs"]
mod set_firmware_environment_variable;
pub use set_firmware_environment_variable::*;

#[path = "../set_environment_variable.rs"]
mod set_environment_variable;
pub use set_environment_variable::*;

#[path = "../get_version_ex.rs"]
mod get_version_ex;
pub use get_version_ex::*;

#[path = "../get_number_format.rs"]
mod get_number_format;
pub use get_number_format::*;

#[path = "../get_geo_info.rs"]
mod get_geo_info;
pub use get_geo_info::*;

#[path = "../get_firmware_environment_variable.rs"]
mod get_firmware_environment_variable;
pub use get_firmware_environment_variable::*;

#[path = "../get_currency_format.rs"]
mod get_currency_format;
pub use get_currency_format::*;

#[path = "../get_acp.rs"]
mod get_acp;
pub use get_acp::*;

#[path = "../get_cp_info.rs"]
mod get_cp_info;
pub use get_cp_info::*;

#[path = "../get_local_time.rs"]
mod get_local_time;
pub use get_local_time::*;

#[path = "../get_native_system_info.rs"]
mod get_native_system_info;
pub use get_native_system_info::*;

#[path = "../get_oemcp.rs"]
mod get_oemcp;
pub use get_oemcp::*;

#[path = "../get_system_info.rs"]
mod get_system_info;
pub use get_system_info::*;

#[path = "../get_system_time.rs"]
mod get_system_time;
pub use get_system_time::*;

#[path = "../get_tick_count.rs"]
mod get_tick_count;
pub use get_tick_count::*;

#[path = "../get_time_zone_information.rs"]
mod get_time_zone_information;
pub use get_time_zone_information::*;

#[path = "../get_user_default_lang_id.rs"]
mod get_user_default_lang_id;
pub use get_user_default_lang_id::*;

#[path = "../get_user_default_ui_language.rs"]
mod get_user_default_ui_language;
pub use get_user_default_ui_language::*;

#[path = "../get_version.rs"]
mod get_version;
pub use get_version::*;

#[path = "../is_valid_code_page.rs"]
mod is_valid_code_page;
pub use is_valid_code_page::*;

#[path = "../is_valid_locale.rs"]
mod is_valid_locale;
pub use is_valid_locale::*;

#[path = "../system_time_to_tz_specific_local_time.rs"]
mod system_time_to_tz_specific_local_time;
pub use system_time_to_tz_specific_local_time::*;

#[path = "../adjust_calendar_date.rs"]
mod adjust_calendar_date;
pub use adjust_calendar_date::*;

#[path = "../calibrate_timer.rs"]
mod calibrate_timer;
pub use calibrate_timer::*;

#[path = "../cancel_timer_queue_timer.rs"]
mod cancel_timer_queue_timer;
pub use cancel_timer_queue_timer::*;

#[path = "../compare_calendar_dates.rs"]
mod compare_calendar_dates;
pub use compare_calendar_dates::*;

#[path = "../convert_cal_date_time_to_system_time.rs"]
mod convert_cal_date_time_to_system_time;
pub use convert_cal_date_time_to_system_time::*;

#[path = "../convert_system_time_to_cal_date_time.rs"]
mod convert_system_time_to_cal_date_time;
pub use convert_system_time_to_cal_date_time::*;

#[path = "../enum_system_geo_id.rs"]
mod enum_system_geo_id;
pub use enum_system_geo_id::*;

#[path = "../enum_system_geo_names.rs"]
mod enum_system_geo_names;
pub use enum_system_geo_names::*;

#[path = "../get_calendar_date_format.rs"]
mod get_calendar_date_format;
pub use get_calendar_date_format::*;

#[path = "../get_calendar_date_format_ex.rs"]
mod get_calendar_date_format_ex;
pub use get_calendar_date_format_ex::*;

#[path = "../get_calendar_days_in_month.rs"]
mod get_calendar_days_in_month;
pub use get_calendar_days_in_month::*;

#[path = "../get_calendar_difference_in_days.rs"]
mod get_calendar_difference_in_days;
pub use get_calendar_difference_in_days::*;

#[path = "../get_calendar_months_in_year.rs"]
mod get_calendar_months_in_year;
pub use get_calendar_months_in_year::*;

#[path = "../get_calendar_supported_date_range.rs"]
mod get_calendar_supported_date_range;
pub use get_calendar_supported_date_range::*;

#[path = "../get_calendar_week_number.rs"]
mod get_calendar_week_number;
pub use get_calendar_week_number::*;

#[path = "../get_date_format_a_worker.rs"]
mod get_date_format_a_worker;
pub use get_date_format_a_worker::*;

#[path = "../get_date_format_w_worker.rs"]
mod get_date_format_w_worker;
pub use get_date_format_w_worker::*;

#[path = "../get_firmware_type.rs"]
mod get_firmware_type;
pub use get_firmware_type::*;

#[path = "../get_geo_info_ex.rs"]
mod get_geo_info_ex;
pub use get_geo_info_ex::*;

#[path = "../get_numa_node_number_from_handle.rs"]
mod get_numa_node_number_from_handle;
pub use get_numa_node_number_from_handle::*;

#[path = "../get_system_dep_policy.rs"]
mod get_system_dep_policy;
pub use get_system_dep_policy::*;

#[path = "../get_system_power_status.rs"]
mod get_system_power_status;
pub use get_system_power_status::*;

#[path = "../get_system_registry_quota.rs"]
mod get_system_registry_quota;
pub use get_system_registry_quota::*;

#[path = "../get_time_format_w_worker.rs"]
mod get_time_format_w_worker;
pub use get_time_format_w_worker::*;

#[path = "../get_user_default_geo_name.rs"]
mod get_user_default_geo_name;
pub use get_user_default_geo_name::*;

#[path = "../get_user_geo_id.rs"]
mod get_user_geo_id;
pub use get_user_geo_id::*;

#[path = "../is_calendar_leap_day.rs"]
mod is_calendar_leap_day;
pub use is_calendar_leap_day::*;

#[path = "../is_calendar_leap_month.rs"]
mod is_calendar_leap_month;
pub use is_calendar_leap_month::*;

#[path = "../is_calendar_leap_year.rs"]
mod is_calendar_leap_year;
pub use is_calendar_leap_year::*;

#[path = "../is_caller_admin_or_system.rs"]
mod is_caller_admin_or_system;
pub use is_caller_admin_or_system::*;

#[path = "../is_system_luid.rs"]
mod is_system_luid;
pub use is_system_luid::*;

#[path = "../is_system_resume_automatic.rs"]
mod is_system_resume_automatic;
pub use is_system_resume_automatic::*;

#[path = "../is_valid_cal_date_time.rs"]
mod is_valid_cal_date_time;
pub use is_valid_cal_date_time::*;

#[path = "../notify_ui_language_change.rs"]
mod notify_ui_language_change;
pub use notify_ui_language_change::*;

#[path = "../set_system_power_state.rs"]
mod set_system_power_state;
pub use set_system_power_state::*;

#[path = "../set_system_time_adjustment.rs"]
mod set_system_time_adjustment;
pub use set_system_time_adjustment::*;

#[path = "../set_timer_queue_timer.rs"]
mod set_timer_queue_timer;
pub use set_timer_queue_timer::*;

#[path = "../set_user_geo_id.rs"]
mod set_user_geo_id;
pub use set_user_geo_id::*;

#[path = "../set_user_geo_name.rs"]
mod set_user_geo_name;
pub use set_user_geo_name::*;

#[path = "../time_begin_period.rs"]
mod time_begin_period;

#[path = "../time_end_period.rs"]
mod time_end_period;
pub use time_end_period::*;

#[path = "../time_get_dev_caps.rs"]
mod time_get_dev_caps;
pub use time_get_dev_caps::*;

#[path = "../time_get_time.rs"]
mod time_get_time;
pub use time_get_time::*;

#[path = "../time_init.rs"]
mod time_init;
pub use time_init::*;

#[path = "../update_calendar_day_of_week.rs"]
mod update_calendar_day_of_week;
pub use update_calendar_day_of_week::*;

#[path = "../system_time_to_file_time.rs"]
mod system_time_to_file_time;
pub use system_time_to_file_time::*;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "FormatMessageA" => kernel32::FormatMessageA(emu),
        "FormatMessageW" => kernel32::FormatMessageW(emu),
        "GetVersionExA" => kernel32::GetVersionExA(emu),
        "GetLastError" => kernel32::GetLastError(emu),
        "GetNativeSystemInfo" => kernel32::GetNativeSystemInfo(emu),
        "GetSystemInfo" => kernel32::GetSystemInfo(emu),
        "GetVersion" => kernel32::GetVersion(emu),
        "GetVersionExW" => kernel32::GetVersionExW(emu),
        "SetErrorMode" => kernel32::SetErrorMode(emu),
        "SetLastError" => kernel32::SetLastError(emu),
        "VerifyVersionInfoW" => kernel32::VerifyVersionInfoW(emu),
        "AddLocalAlternateComputerNameW" => kernel32::AddLocalAlternateComputerNameW(emu),
        "BaseCheckElevation" => kernel32::BaseCheckElevation(emu),
        "BaseDllInitialize" => kernel32::BaseDllInitialize(emu),
        "BaseSetLastNTError" => kernel32::BaseSetLastNTError(emu),
        "BaseUpdateVDMEntry" => kernel32::BaseUpdateVDMEntry(emu),
        "BasepAccumulateIoRateControlInformationBufferSize" => {
            kernel32::BasepAccumulateIoRateControlInformationBufferSize(emu)
        }
        "BasepCheckWebBladeHashes" => kernel32::BasepCheckWebBladeHashes(emu),
        "BasepCheckWinSaferRestrictions" => kernel32::BasepCheckWinSaferRestrictions(emu),
        "BasepCopyEncryption" => kernel32::BasepCopyEncryption(emu),
        "BasepGetExeArchType" => kernel32::BasepGetExeArchType(emu),
        "BasepGetMiniVersionForCreate" => kernel32::BasepGetMiniVersionForCreate(emu),
        "BasepInitializeTermsrvFpns" => kernel32::BasepInitializeTermsrvFpns(emu),
        "BasepIsTestSigningEnabled" => kernel32::BasepIsTestSigningEnabled(emu),
        "BasepReportFault" => kernel32::BasepReportFault(emu),
        "BasepSxsCreateStreams" => kernel32::BasepSxsCreateStreams(emu),
        "BeepImplementation" => kernel32::BeepImplementation(emu),
        "ByteMatch" => kernel32::ByteMatch(emu),
        "CheckElevation" => kernel32::CheckElevation(emu),
        "ConvertNLSDayOfWeekToWin32DayOfWeek" => kernel32::ConvertNLSDayOfWeekToWin32DayOfWeek(emu),
        "DnsHostnameToComputerNameA" => kernel32::DnsHostnameToComputerNameA(emu),
        "DnsHostnameToComputerNameW" => kernel32::DnsHostnameToComputerNameW(emu),
        "EnumerateLocalComputerNamesA" => kernel32::EnumerateLocalComputerNamesA(emu),
        "EnumerateLocalComputerNamesW" => kernel32::EnumerateLocalComputerNamesW(emu),
        "GetCompatFlags" => kernel32::GetCompatFlags(emu),
        "GetConfigDialogName" => kernel32::GetConfigDialogName(emu),
        "GetCorruptDetectionState" => kernel32::GetCorruptDetectionState(emu),
        "GetDevicePowerState" => kernel32::GetDevicePowerState(emu),
        "GetExpandedNameA" => kernel32::GetExpandedNameA(emu),
        "GetExpandedNameW" => kernel32::GetExpandedNameW(emu),
        "GetFirmwareType" => kernel32::GetFirmwareType(emu),
        "GetNumaProximityNode" => kernel32::GetNumaProximityNode(emu),
        "GetSystemPowerStatus" => kernel32::GetSystemPowerStatus(emu),
        "InternalCoStdMarshalObject" => kernel32::InternalCoStdMarshalObject(emu),
        "IsCallerAdminOrSystem" => kernel32::IsCallerAdminOrSystem(emu),
        "IsFusionFullySupported" => kernel32::IsFusionFullySupported(emu),
        "IsOOBESupported" => kernel32::IsOOBESupported(emu),
        "IsSystemLUID" => kernel32::IsSystemLUID(emu),
        "IsSystemResumeAutomatic" => kernel32::IsSystemResumeAutomatic(emu),
        "IsTerminalServerCompatible" => kernel32::IsTerminalServerCompatible(emu),
        "MachinePrefersNumericFirstYear" => kernel32::MachinePrefersNumericFirstYear(emu),
        "MulDiv" => kernel32::MulDiv(emu),
        "NonInstallerDefault" => kernel32::NonInstallerDefault(emu),
        "NtWow64CsrBaseCheckRunApp" => kernel32::NtWow64CsrBaseCheckRunApp(emu),
        "NtWow64CsrBasepDefineDosDevice" => kernel32::NtWow64CsrBasepDefineDosDevice(emu),
        "NtWow64CsrBasepNlsGetUserInfo" => kernel32::NtWow64CsrBasepNlsGetUserInfo(emu),
        "NtWow64CsrBasepNlsUpdateCacheCount" => kernel32::NtWow64CsrBasepNlsUpdateCacheCount(emu),
        "NtWow64CsrBasepSoundSentryNotification" => {
            kernel32::NtWow64CsrBasepSoundSentryNotification(emu)
        }
        "OOBEComplete" => kernel32::OOBEComplete(emu),
        "OOBECompleteWnfCallback" => kernel32::OOBECompleteWnfCallback(emu),
        "OOBECompleteWnfQueryCallback" => kernel32::OOBECompleteWnfQueryCallback(emu),
        "OOBECompleteWnfWaitCallback" => kernel32::OOBECompleteWnfWaitCallback(emu),
        "OpenSortIdKey" => kernel32::OpenSortIdKey(emu),
        "PowerClearRequest" => kernel32::PowerClearRequest(emu),
        "PowerCreateRequest" => kernel32::PowerCreateRequest(emu),
        "PowerSetRequest" => kernel32::PowerSetRequest(emu),
        "RemoveLocalAlternateComputerNameW" => kernel32::RemoveLocalAlternateComputerNameW(emu),
        "ReplacePartitionUnit" => kernel32::ReplacePartitionUnit(emu),
        "RequestWakeupLatency" => kernel32::RequestWakeupLatency(emu),
        "SetLocalPrimaryComputerNameW" => kernel32::SetLocalPrimaryComputerNameW(emu),
        "SetMessageWaitingIndicator" => kernel32::SetMessageWaitingIndicator(emu),
        "SetSystemPowerState" => kernel32::SetSystemPowerState(emu),
        "SXSManifest" => kernel32::SXSManifest(emu),
        "ULongAdd" => kernel32::ULongAdd(emu),
        "ULongLongToULong" => kernel32::ULongLongToULong(emu),
        "ULongMult" => kernel32::ULongMult(emu),
        "VDMOperationStarted" => kernel32::VDMOperationStarted(emu),
        "VerifyVersionInfoA" => kernel32::VerifyVersionInfoA(emu),
        "AddLocalAlternateComputerNameA" => kernel32::AddLocalAlternateComputerNameA(emu),
        "BasepCheckPplSupport" => kernel32::BasepCheckPplSupport(emu),
        "Beep" => kernel32::Beep(emu),
        "CancelDeviceWakeupRequest" => kernel32::CancelDeviceWakeupRequest(emu),
        "CeipIsOptedIn" => kernel32::CeipIsOptedIn(emu),
        "CheckElevationEnabled" => kernel32::CheckElevationEnabled(emu),
        "CloseProfileUserMapping" => kernel32::CloseProfileUserMapping(emu),
        "CmdBatNotification" => kernel32::CmdBatNotification(emu),
        "CtrlRoutine" => kernel32::CtrlRoutine(emu),
        "DnsHostnameToComputerNameExW" => kernel32::DnsHostnameToComputerNameExW(emu),
        "EnumSystemFirmwareTables" => kernel32::EnumSystemFirmwareTables(emu),
        "GetCachedSigningLevel" => kernel32::GetCachedSigningLevel(emu),
        "GetComputerNameExA" => kernel32::GetComputerNameExA(emu),
        "GetComputerNameExW" => kernel32::GetComputerNameExW(emu),
        "GetEnabledXStateFeatures" => kernel32::GetEnabledXStateFeatures(emu),
        "GetErrorMode" => kernel32::GetErrorMode(emu),
        "GetMachineTypeAttributes" => kernel32::GetMachineTypeAttributes(emu),
        "GetNumaHighestNodeNumber" => kernel32::GetNumaHighestNodeNumber(emu),
        "GetNumaProximityNodeEx" => kernel32::GetNumaProximityNodeEx(emu),
        "GetProductInfo" => kernel32::GetProductInfo(emu),
        "GetSystemAppDataKey" => kernel32::GetSystemAppDataKey(emu),
        "GetSystemCpuSetInformation" => kernel32::GetSystemCpuSetInformation(emu),
        "GetSystemFirmwareTable" => kernel32::GetSystemFirmwareTable(emu),
        "GetVDMCurrentDirectories" => kernel32::GetVDMCurrentDirectories(emu),
        "GetXStateFeaturesMask" => kernel32::GetXStateFeaturesMask(emu),
        "InstallELAMCertificateInfo" => kernel32::InstallELAMCertificateInfo(emu),
        "IsNativeVhdBoot" => kernel32::IsNativeVhdBoot(emu),
        "IsWow64GuestMachineSupported" => kernel32::IsWow64GuestMachineSupported(emu),
        "LocateXStateFeature" => kernel32::LocateXStateFeature(emu),
        "LogUnexpectedCodepath" => kernel32::LogUnexpectedCodepath(emu),
        "OpenProfileUserMapping" => kernel32::OpenProfileUserMapping(emu),
        "RemoveLocalAlternateComputerNameA" => kernel32::RemoveLocalAlternateComputerNameA(emu),
        "RequestDeviceWakeup" => kernel32::RequestDeviceWakeup(emu),
        "RestoreLastError" => kernel32::RestoreLastError(emu),
        "SetCachedSigningLevel" => kernel32::SetCachedSigningLevel(emu),
        "SetComputerNameA" => kernel32::SetComputerNameA(emu),
        "SetComputerNameEx2W" => kernel32::SetComputerNameEx2W(emu),
        "SetComputerNameExA" => kernel32::SetComputerNameExA(emu),
        "SetComputerNameExW" => kernel32::SetComputerNameExW(emu),
        "SetComputerNameW" => kernel32::SetComputerNameW(emu),
        "SetLocalPrimaryComputerNameA" => kernel32::SetLocalPrimaryComputerNameA(emu),
        "SetTermsrvAppInstallMode" => kernel32::SetTermsrvAppInstallMode(emu),
        "SetVDMCurrentDirectories" => kernel32::SetVDMCurrentDirectories(emu),
        "SetXStateFeaturesMask" => kernel32::SetXStateFeaturesMask(emu),
        "UTRegister" => kernel32::UTRegister(emu),
        "UTUnRegister" => kernel32::UTUnRegister(emu),
        "VerSetConditionMask" => kernel32::VerSetConditionMask(emu),
        "RtlIsEcCode" => kernel32::RtlIsEcCode(emu),
        _ => return false,
    }
    true
}
