use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "CompareFileTime" => kernelbase::CompareFileTime(emu),
        "DosDateTimeToFileTime" => kernelbase::DosDateTimeToFileTime(emu),
        "FileTimeToDosDateTime" => kernelbase::FileTimeToDosDateTime(emu),
        "DeleteTimerQueue" => kernelbase::DeleteTimerQueue(emu),
        "FileTimeToLocalFileTime" => kernelbase::FileTimeToLocalFileTime(emu),
        "FileTimeToSystemTime" => kernelbase::FileTimeToSystemTime(emu),
        "GetCalendarInfoW" => kernelbase::GetCalendarInfoW(emu),
        "GetLocalTime" => kernelbase::GetLocalTime(emu),
        "GetSystemTime" => kernelbase::GetSystemTime(emu),
        "GetSystemTimeAsFileTime" => kernelbase::GetSystemTimeAsFileTime(emu),
        "GetSystemTimePreciseAsFileTime" => kernelbase::GetSystemTimePreciseAsFileTime(emu),
        "GetTickCount" => kernelbase::GetTickCount(emu),
        "GetTickCount64" => kernelbase::GetTickCount64(emu),
        "GetTimeZoneInformation" => kernelbase::GetTimeZoneInformation(emu),
        "GetTimeZoneInformationForYear" => kernelbase::GetTimeZoneInformationForYear(emu),
        "LocalFileTimeToFileTime" => kernelbase::LocalFileTimeToFileTime(emu),
        "LocalFileTimeToLocalSystemTime" => kernelbase::LocalFileTimeToLocalSystemTime(emu),
        "LocalSystemTimeToLocalFileTime" => kernelbase::LocalSystemTimeToLocalFileTime(emu),
        "QueryPerformanceCounter" => kernelbase::QueryPerformanceCounter(emu),
        "QueryPerformanceFrequency" => kernelbase::QueryPerformanceFrequency(emu),
        "SetSystemTimeAdjustment" => kernelbase::SetSystemTimeAdjustment(emu),
        "SystemTimeToFileTime" => kernelbase::SystemTimeToFileTime(emu),
        "SystemTimeToTzSpecificLocalTime" => kernelbase::SystemTimeToTzSpecificLocalTime(emu),
        "SystemTimeToTzSpecificLocalTimeEx" => kernelbase::SystemTimeToTzSpecificLocalTimeEx(emu),
        "TzSpecificLocalTimeToSystemTimeEx" => kernelbase::TzSpecificLocalTimeToSystemTimeEx(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
