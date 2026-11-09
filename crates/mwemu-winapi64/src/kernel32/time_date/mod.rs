use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "FileTimeToSystemTime" => kernel32::FileTimeToSystemTime(emu),
        "GetFileTime" => kernel32::GetFileTime(emu),
        "SetFileTime" => kernel32::SetFileTime(emu),
        "GetLocalTime" => kernel32::GetLocalTime(emu),
        "GetSystemTime" => kernel32::GetSystemTime(emu),
        "GetSystemTimeAsFileTime" => kernel32::GetSystemTimeAsFileTime(emu),
        "GetSystemTimePreciseAsFileTime" => kernel32::GetSystemTimePreciseAsFileTime(emu),
        "GetTickCount" => kernel32::GetTickCount(emu),
        "GetTickCount64" => kernel32::GetTickCount64(emu),
        "GetTimeZoneInformation" => kernel32::GetTimeZoneInformation(emu),
        "GetTimeZoneInformationForYear" => kernel32::GetTimeZoneInformationForYear(emu),
        "QueryPerformanceCounter" => kernel32::QueryPerformanceCounter(emu),
        "QueryPerformanceFrequency" => kernel32::QueryPerformanceFrequency(emu),
        "SystemTimeToFileTime" => kernel32::SystemTimeToFileTime(emu),
        "SystemTimeToTzSpecificLocalTime" => kernel32::SystemTimeToTzSpecificLocalTime(emu),
        "GetDateFormatW" => kernel32::GetDateFormatW(emu),
        "GetTimeFormatW" => kernel32::GetTimeFormatW(emu),
        _ => return false,
    }
    true
}
