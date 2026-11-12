use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod get_version_ex;
pub use get_version_ex::{GetVersionExA, GetVersionExW};

mod get_acp;
pub use get_acp::GetACP;

mod get_cp_info;
pub use get_cp_info::GetCPInfo;

mod get_local_time;
pub use get_local_time::GetLocalTime;

mod get_locale_info;
pub use get_locale_info::{GetLocaleInfoA, GetLocaleInfoW};

mod get_native_system_info;
pub use get_native_system_info::GetNativeSystemInfo;

mod get_system_firmware_table;
pub use get_system_firmware_table::GetSystemFirmwareTable;

mod get_system_info;
pub use get_system_info::GetSystemInfo;

mod get_system_time;
pub use get_system_time::GetSystemTime;

mod get_tick_count;
pub use get_tick_count::GetTickCount;

mod get_time_zone_information;
pub use get_time_zone_information::GetTimeZoneInformation;

mod get_time_zone_information_for_year;
pub use get_time_zone_information_for_year::GetTimeZoneInformationForYear;

mod get_user_default_lang_id;
pub use get_user_default_lang_id::GetUserDefaultLangID;

mod get_version;
pub use get_version::GetVersion;

mod system_time_to_tz_specific_local_time;
pub use system_time_to_tz_specific_local_time::SystemTimeToTzSpecificLocalTime;

mod get_tick_count64;
pub use get_tick_count64::GetTickCount64;

mod query_performance_frequency;
pub use query_performance_frequency::QueryPerformanceFrequency;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "FormatMessageA" => kernel32::FormatMessageA(emu),
        "FormatMessageW" => kernel32::FormatMessageW(emu),
        "GetLastError" => kernel32::GetLastError(emu),
        "GetNativeSystemInfo" => kernel32::GetNativeSystemInfo(emu),
        "GetSystemFirmwareTable" => kernel32::GetSystemFirmwareTable(emu),
        "GetSystemInfo" => kernel32::GetSystemInfo(emu),
        "GetVersion" => kernel32::GetVersion(emu),
        "GetVersionExA" => kernel32::GetVersionExA(emu),
        "GetVersionExW" => kernel32::GetVersionExW(emu),
        "SetErrorMode" => kernel32::SetErrorMode(emu),
        "SetLastError" => kernel32::SetLastError(emu),
        "VerifyVersionInfoA" => kernel32::VerifyVersionInfoA(emu),
        "VerifyVersionInfoW" => kernel32::VerifyVersionInfoW(emu),
        "VerSetConditionMask" => kernel32::VerSetConditionMask(emu),
        "Beep" => kernel32::Beep(emu),
        _ => return false,
    }
    true
}
