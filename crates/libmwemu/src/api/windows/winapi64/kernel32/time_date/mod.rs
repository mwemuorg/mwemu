use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::libraries::kernel32::system::{timeGetDevCaps, timeGetTime};
use crate::winapi::winapi64::kernel32;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "timeGetDevCaps" => timeGetDevCaps(emu),
        "timeGetTime" => timeGetTime(emu),
        "CompareFileTime" => kernel32::CompareFileTime(emu),
        "DosDateTimeToFileTime" => kernel32::DosDateTimeToFileTime(emu),
        "FileTimeToDosDateTime" => kernel32::FileTimeToDosDateTime(emu),
        "FileTimeToLocalFileTime" => kernel32::FileTimeToLocalFileTime(emu),
        "LocalFileTimeToFileTime" => kernel32::LocalFileTimeToFileTime(emu),
        "FileTimeToSystemTime" => kernel32::FileTimeToSystemTime(emu),
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
        "GetDateFormatA" => kernel32::GetDateFormatA(emu),
        "GetDateFormatW" => kernel32::GetDateFormatW(emu),
        "GetTimeFormatA" => kernel32::GetTimeFormatA(emu),
        "GetTimeFormatW" => kernel32::GetTimeFormatW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
