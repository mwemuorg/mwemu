pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    GetCPInfo, GetLocaleInfoA, GetLocaleInfoW, GetNativeSystemInfo, GetSystemFirmwareTable,
    GetSystemInfo, GetTimeZoneInformationForYear, GetVersion, SystemTimeToTzSpecificLocalTime,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::libraries::kernel32::misc::MulDiv;
use crate::winapi::winapi64::kernel32;

pub use crate::api::windows::libraries::kernel32::GetTimeZoneInformation;
pub use crate::api::windows::libraries::kernel32::system::{GetACP, GetUserDefaultLangID};

pub use crate::winapi::common::ntdll::{GetVersionExA, GetVersionExW};

pub use crate::winapi::libraries::kernel32::{GetLocalTime, GetSystemTime};

mod get_tick_count;
pub use get_tick_count::GetTickCount;

mod get_tick_count64;
pub use get_tick_count64::GetTickCount64;

mod query_performance_frequency;
pub use query_performance_frequency::QueryPerformanceFrequency;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "MulDiv" => MulDiv(emu),
        "FormatMessageA" => kernel32::FormatMessageA(emu),
        "FormatMessageW" => kernel32::FormatMessageW(emu),
        "GetLastError" => kernel32::GetLastError(emu),
        "GetNativeSystemInfo" => kernel32::GetNativeSystemInfo(emu),
        "GetSystemFirmwareTable" => kernel32::GetSystemFirmwareTable(emu),
        "GetSystemInfo" => kernel32::GetSystemInfo(emu),
        "GetVersion" => kernel32::GetVersion(emu),
        "GetVersionExA" => kernel32::GetVersionExA(emu),
        "GetVersionExW" => kernel32::GetVersionExW(emu),
        "SetLastError" => kernel32::SetLastError(emu),
        "VerifyVersionInfoA" => kernel32::VerifyVersionInfoA(emu),
        "VerifyVersionInfoW" => kernel32::VerifyVersionInfoW(emu),
        "VerSetConditionMask" => kernel32::VerSetConditionMask(emu),
        "Beep" => kernel32::Beep(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
