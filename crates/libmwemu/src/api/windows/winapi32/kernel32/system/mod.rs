use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

mod set_locale_info;
pub use set_locale_info::{SetLocaleInfoA, SetLocaleInfoW};

mod set_calendar_info;
pub use set_calendar_info::{SetCalendarInfoA, SetCalendarInfoW};

mod get_environment_variable;
pub use get_environment_variable::{GetEnvironmentVariableA, GetEnvironmentVariableW};

mod get_calendar_info;
pub use get_calendar_info::{GetCalendarInfoA, GetCalendarInfoW};

mod enum_ui_languages;
pub use enum_ui_languages::{EnumUILanguagesA, EnumUILanguagesW};

mod enum_time_formats;
pub use enum_time_formats::{EnumTimeFormatsA, EnumTimeFormatsW};

mod enum_system_language_groups;
pub use enum_system_language_groups::{EnumSystemLanguageGroupsA, EnumSystemLanguageGroupsW};

mod enum_system_code_pages;
pub use enum_system_code_pages::{EnumSystemCodePagesA, EnumSystemCodePagesW};

mod enum_language_group_locales;
pub use enum_language_group_locales::{EnumLanguageGroupLocalesA, EnumLanguageGroupLocalesW};

mod enum_date_formats_ex;
pub use enum_date_formats_ex::{EnumDateFormatsExA, EnumDateFormatsExW};

mod enum_date_formats;
pub use enum_date_formats::{EnumDateFormatsA, EnumDateFormatsW};

mod enum_calendar_info_ex;
pub use enum_calendar_info_ex::{EnumCalendarInfoExA, EnumCalendarInfoExW};

mod enum_calendar_info;
pub use enum_calendar_info::{EnumCalendarInfoA, EnumCalendarInfoW};

mod verify_version_info;
pub use verify_version_info::{VerifyVersionInfoA, VerifyVersionInfoW};

mod set_firmware_environment_variable;
pub use set_firmware_environment_variable::{
    SetFirmwareEnvironmentVariableA, SetFirmwareEnvironmentVariableW,
};

mod set_environment_variable;
pub use set_environment_variable::{SetEnvironmentVariableA, SetEnvironmentVariableW};

mod get_version_ex;
pub use get_version_ex::{GetVersionExA, GetVersionExW};

mod get_number_format;
pub use get_number_format::{GetNumberFormatA, GetNumberFormatW};

mod get_geo_info;
pub use get_geo_info::{GetGeoInfoA, GetGeoInfoW};

mod get_firmware_environment_variable;
pub use get_firmware_environment_variable::{
    GetFirmwareEnvironmentVariableA, GetFirmwareEnvironmentVariableW,
};

mod get_currency_format;
pub use get_currency_format::{GetCurrencyFormatA, GetCurrencyFormatW};

mod get_acp;
pub use get_acp::GetACP;

mod get_cp_info;
pub use get_cp_info::GetCPInfo;

mod get_local_time;
pub use get_local_time::GetLocalTime;

mod get_native_system_info;
pub use get_native_system_info::GetNativeSystemInfo;

mod get_oemcp;
pub use get_oemcp::GetOEMCP;

mod get_system_info;
pub use get_system_info::GetSystemInfo;

mod get_system_time;
pub use get_system_time::GetSystemTime;

mod get_tick_count;
pub use get_tick_count::GetTickCount;

mod get_time_zone_information;
pub use get_time_zone_information::GetTimeZoneInformation;

mod get_user_default_lang_id;
pub use get_user_default_lang_id::GetUserDefaultLangID;

mod get_user_default_ui_language;
pub use get_user_default_ui_language::GetUserDefaultUILanguage;

mod get_version;
pub use get_version::GetVersion;

mod is_valid_code_page;
pub use is_valid_code_page::IsValidCodePage;

mod is_valid_locale;
pub use is_valid_locale::IsValidLocale;

mod system_time_to_tz_specific_local_time;
pub use system_time_to_tz_specific_local_time::SystemTimeToTzSpecificLocalTime;

mod adjust_calendar_date;
pub use adjust_calendar_date::AdjustCalendarDate;

mod calibrate_timer;
pub use calibrate_timer::CalibrateTimer;

mod cancel_timer_queue_timer;
pub use cancel_timer_queue_timer::CancelTimerQueueTimer;

mod compare_calendar_dates;
pub use compare_calendar_dates::CompareCalendarDates;

mod convert_cal_date_time_to_system_time;
pub use convert_cal_date_time_to_system_time::ConvertCalDateTimeToSystemTime;

mod convert_system_time_to_cal_date_time;
pub use convert_system_time_to_cal_date_time::ConvertSystemTimeToCalDateTime;

mod enum_system_geo_id;
pub use enum_system_geo_id::EnumSystemGeoID;

mod enum_system_geo_names;
pub use enum_system_geo_names::EnumSystemGeoNames;

mod get_calendar_date_format;
pub use get_calendar_date_format::GetCalendarDateFormat;

mod get_calendar_date_format_ex;
pub use get_calendar_date_format_ex::GetCalendarDateFormatEx;

mod get_calendar_days_in_month;
pub use get_calendar_days_in_month::GetCalendarDaysInMonth;

mod get_calendar_difference_in_days;
pub use get_calendar_difference_in_days::GetCalendarDifferenceInDays;

mod get_calendar_months_in_year;
pub use get_calendar_months_in_year::GetCalendarMonthsInYear;

mod get_calendar_supported_date_range;
pub use get_calendar_supported_date_range::GetCalendarSupportedDateRange;

mod get_calendar_week_number;
pub use get_calendar_week_number::GetCalendarWeekNumber;

mod get_date_format_a_worker;
pub use get_date_format_a_worker::GetDateFormatAWorker;

mod get_date_format_w_worker;
pub use get_date_format_w_worker::GetDateFormatWWorker;

mod get_firmware_type;
pub use get_firmware_type::GetFirmwareType;

mod get_geo_info_ex;
pub use get_geo_info_ex::GetGeoInfoEx;

mod get_numa_node_number_from_handle;
pub use get_numa_node_number_from_handle::GetNumaNodeNumberFromHandle;

mod get_system_dep_policy;
pub use get_system_dep_policy::GetSystemDEPPolicy;

mod get_system_power_status;
pub use get_system_power_status::GetSystemPowerStatus;

mod get_system_registry_quota;
pub use get_system_registry_quota::GetSystemRegistryQuota;

mod get_time_format_w_worker;
pub use get_time_format_w_worker::GetTimeFormatWWorker;

mod get_user_default_geo_name;
pub use get_user_default_geo_name::GetUserDefaultGeoName;

mod get_user_geo_id;
pub use get_user_geo_id::GetUserGeoID;

mod is_calendar_leap_day;
pub use is_calendar_leap_day::IsCalendarLeapDay;

mod is_calendar_leap_month;
pub use is_calendar_leap_month::IsCalendarLeapMonth;

mod is_calendar_leap_year;
pub use is_calendar_leap_year::IsCalendarLeapYear;

mod is_caller_admin_or_system;
pub use is_caller_admin_or_system::IsCallerAdminOrSystem;

mod is_system_luid;
pub use is_system_luid::IsSystemLUID;

mod is_system_resume_automatic;
pub use is_system_resume_automatic::IsSystemResumeAutomatic;

mod is_valid_cal_date_time;
pub use is_valid_cal_date_time::IsValidCalDateTime;

mod notify_ui_language_change;
pub use notify_ui_language_change::NotifyUILanguageChange;

mod set_system_power_state;
pub use set_system_power_state::SetSystemPowerState;

mod set_system_time_adjustment;
pub use set_system_time_adjustment::SetSystemTimeAdjustment;

mod set_timer_queue_timer;
pub use set_timer_queue_timer::SetTimerQueueTimer;

mod set_user_geo_id;
pub use set_user_geo_id::SetUserGeoID;

mod set_user_geo_name;
pub use set_user_geo_name::SetUserGeoName;

mod time_begin_period;

mod time_end_period;
pub use time_end_period::timeEndPeriod;

mod time_get_dev_caps;
pub use time_get_dev_caps::timeGetDevCaps;

mod time_get_time;
pub use time_get_time::timeGetTime;

mod time_init;
pub use time_init::TimeInit;

mod update_calendar_day_of_week;
pub use update_calendar_day_of_week::UpdateCalendarDayOfWeek;

mod system_time_to_file_time;
pub use system_time_to_file_time::SystemTimeToFileTime;

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
