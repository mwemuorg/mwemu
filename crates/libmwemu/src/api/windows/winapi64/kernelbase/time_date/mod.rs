use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "ConvertAuxiliaryCounterToPerformanceCounter" => {
            kernelbase::ConvertAuxiliaryCounterToPerformanceCounter(emu)
        }
        "ConvertPerformanceCounterToAuxiliaryCounter" => {
            kernelbase::ConvertPerformanceCounterToAuxiliaryCounter(emu)
        }
        "EnumDynamicTimeZoneInformation" => kernelbase::EnumDynamicTimeZoneInformation(emu),
        "GetCalendar" => kernelbase::GetCalendar(emu),
        "GetDynamicTimeZoneInformationEffectiveYears" => {
            kernelbase::GetDynamicTimeZoneInformationEffectiveYears(emu)
        }
        "GetRoamingLastObservedChangeTime" => kernelbase::GetRoamingLastObservedChangeTime(emu),
        "GetSystemLeapSecondInformation" => kernelbase::GetSystemLeapSecondInformation(emu),
        "GetSystemTimeAdjustmentPrecise" => kernelbase::GetSystemTimeAdjustmentPrecise(emu),
        "Internal_EnumCalendarInfo" => kernelbase::Internal_EnumCalendarInfo(emu),
        "Internal_EnumDateFormats" => kernelbase::Internal_EnumDateFormats(emu),
        "Internal_EnumTimeFormats" => kernelbase::Internal_EnumTimeFormats(emu),
        "IsTimeZoneRedirectionEnabled" => kernelbase::IsTimeZoneRedirectionEnabled(emu),
        "OverrideRoamingDataModificationTimesInRange" => {
            kernelbase::OverrideRoamingDataModificationTimesInRange(emu)
        }
        "QueryAuxiliaryCounterFrequency" => kernelbase::QueryAuxiliaryCounterFrequency(emu),
        "QueryInterruptTime" => kernelbase::QueryInterruptTime(emu),
        "QueryInterruptTimePrecise" => kernelbase::QueryInterruptTimePrecise(emu),
        "QueryUnbiasedInterruptTimePrecise" => kernelbase::QueryUnbiasedInterruptTimePrecise(emu),
        "SetClientDynamicTimeZoneInformation" => {
            kernelbase::SetClientDynamicTimeZoneInformation(emu)
        }
        "SetClientTimeZoneInformation" => kernelbase::SetClientTimeZoneInformation(emu),
        "SetRoamingLastObservedChangeTime" => kernelbase::SetRoamingLastObservedChangeTime(emu),
        "SetSystemTimeAdjustmentPrecise" => kernelbase::SetSystemTimeAdjustmentPrecise(emu),
        "time" => kernelbase::time(emu),
        "ChangeTimerQueueTimer" => kernelbase::ChangeTimerQueueTimer(emu),
        "CreateTimerQueue" => kernelbase::CreateTimerQueue(emu),
        "CreateTimerQueueTimer" => kernelbase::CreateTimerQueueTimer(emu),
        "DeleteTimerQueue" => kernelbase::DeleteTimerQueue(emu),
        "DeleteTimerQueueEx" => kernelbase::DeleteTimerQueueEx(emu),
        "DeleteTimerQueueTimer" => kernelbase::DeleteTimerQueueTimer(emu),
        "EnumCalendarInfoExEx" => kernelbase::EnumCalendarInfoExEx(emu),
        "EnumCalendarInfoExW" => kernelbase::EnumCalendarInfoExW(emu),
        "EnumCalendarInfoW" => kernelbase::EnumCalendarInfoW(emu),
        "EnumDateFormatsExEx" => kernelbase::EnumDateFormatsExEx(emu),
        "EnumDateFormatsExW" => kernelbase::EnumDateFormatsExW(emu),
        "EnumDateFormatsW" => kernelbase::EnumDateFormatsW(emu),
        "EnumTimeFormatsEx" => kernelbase::EnumTimeFormatsEx(emu),
        "EnumTimeFormatsW" => kernelbase::EnumTimeFormatsW(emu),
        "FileTimeToLocalFileTime" => kernelbase::FileTimeToLocalFileTime(emu),
        "FileTimeToSystemTime" => kernelbase::FileTimeToSystemTime(emu),
        "GetCalendarInfoEx" => kernelbase::GetCalendarInfoEx(emu),
        "GetCalendarInfoW" => kernelbase::GetCalendarInfoW(emu),
        "GetDateFormatA" => kernelbase::GetDateFormatA(emu),
        "GetDateFormatEx" => kernelbase::GetDateFormatEx(emu),
        "GetDateFormatW" => kernelbase::GetDateFormatW(emu),
        "GetDynamicTimeZoneInformation" => kernelbase::GetDynamicTimeZoneInformation(emu),
        "GetFileTime" => kernelbase::GetFileTime(emu),
        "GetLocalTime" => kernelbase::GetLocalTime(emu),
        "GetSystemTime" => kernelbase::GetSystemTime(emu),
        "GetSystemTimeAdjustment" => kernelbase::GetSystemTimeAdjustment(emu),
        "GetSystemTimeAsFileTime" => kernelbase::GetSystemTimeAsFileTime(emu),
        "GetSystemTimePreciseAsFileTime" => kernelbase::GetSystemTimePreciseAsFileTime(emu),
        "GetSystemTimes" => kernelbase::GetSystemTimes(emu),
        "GetTickCount" => kernelbase::GetTickCount(emu),
        "GetTickCount64" => kernelbase::GetTickCount64(emu),
        "GetTimeFormatA" => kernelbase::GetTimeFormatA(emu),
        "GetTimeFormatEx" => kernelbase::GetTimeFormatEx(emu),
        "GetTimeFormatW" => kernelbase::GetTimeFormatW(emu),
        "GetTimeZoneInformation" => kernelbase::GetTimeZoneInformation(emu),
        "GetTimeZoneInformationForYear" => kernelbase::GetTimeZoneInformationForYear(emu),
        "LocalFileTimeToFileTime" => kernelbase::LocalFileTimeToFileTime(emu),
        "LocalFileTimeToLocalSystemTime" => kernelbase::LocalFileTimeToLocalSystemTime(emu),
        "LocalSystemTimeToLocalFileTime" => kernelbase::LocalSystemTimeToLocalFileTime(emu),
        "QueryPerformanceCounter" => kernelbase::QueryPerformanceCounter(emu),
        "QueryPerformanceFrequency" => kernelbase::QueryPerformanceFrequency(emu),
        "QueryUnbiasedInterruptTime" => kernelbase::QueryUnbiasedInterruptTime(emu),
        "SetCalendarInfoW" => kernelbase::SetCalendarInfoW(emu),
        "SetDynamicTimeZoneInformation" => kernelbase::SetDynamicTimeZoneInformation(emu),
        "SetFileTime" => kernelbase::SetFileTime(emu),
        "SetLocalTime" => kernelbase::SetLocalTime(emu),
        "SetSystemTime" => kernelbase::SetSystemTime(emu),
        "SetSystemTimeAdjustment" => kernelbase::SetSystemTimeAdjustment(emu),
        "SetTimeZoneInformation" => kernelbase::SetTimeZoneInformation(emu),
        "SystemTimeToFileTime" => kernelbase::SystemTimeToFileTime(emu),
        "SystemTimeToTzSpecificLocalTime" => kernelbase::SystemTimeToTzSpecificLocalTime(emu),
        "SystemTimeToTzSpecificLocalTimeEx" => kernelbase::SystemTimeToTzSpecificLocalTimeEx(emu),
        "TzSpecificLocalTimeToSystemTime" => kernelbase::TzSpecificLocalTimeToSystemTime(emu),
        "TzSpecificLocalTimeToSystemTimeEx" => kernelbase::TzSpecificLocalTimeToSystemTimeEx(emu),
        _ => return false,
    }
    true
}
