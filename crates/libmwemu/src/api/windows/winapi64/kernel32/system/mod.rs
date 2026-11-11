use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

#[path = "../get_version_ex.rs"]
mod get_version_ex;
pub use get_version_ex::*;

#[path = "../get_acp.rs"]
mod get_acp;
pub use get_acp::*;

#[path = "../get_cp_info.rs"]
mod get_cp_info;
pub use get_cp_info::*;

#[path = "../get_local_time.rs"]
mod get_local_time;
pub use get_local_time::*;

#[path = "../get_locale_info.rs"]
mod get_locale_info;
pub use get_locale_info::*;

#[path = "../get_native_system_info.rs"]
mod get_native_system_info;
pub use get_native_system_info::*;

#[path = "../get_system_firmware_table.rs"]
mod get_system_firmware_table;
pub use get_system_firmware_table::*;

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

#[path = "../get_time_zone_information_for_year.rs"]
mod get_time_zone_information_for_year;
pub use get_time_zone_information_for_year::*;

#[path = "../get_user_default_lang_id.rs"]
mod get_user_default_lang_id;
pub use get_user_default_lang_id::*;

#[path = "../get_version.rs"]
mod get_version;
pub use get_version::*;

#[path = "../system_time_to_tz_specific_local_time.rs"]
mod system_time_to_tz_specific_local_time;
pub use system_time_to_tz_specific_local_time::*;

#[path = "../get_tick_count64.rs"]
mod get_tick_count64;
pub use get_tick_count64::*;

#[path = "../query_performance_frequency.rs"]
mod query_performance_frequency;
pub use query_performance_frequency::*;

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
        "VerifyVersionInfoA" | "VerifyVersionInfoW" => kernel32::VerifyVersionInfoA(emu, api),
        "VerSetConditionMask" => kernel32::VerSetConditionMask(emu),
        "Beep" => kernel32::Beep(emu),
        _ => return false,
    }
    true
}
