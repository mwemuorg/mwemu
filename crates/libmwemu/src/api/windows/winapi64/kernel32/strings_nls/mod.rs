use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod get_calendar_info;
pub use get_calendar_info::{GetCalendarInfoA, GetCalendarInfoW};

mod compare_string;
pub use compare_string::{CompareStringA, CompareStringW};

mod find_act_ctx_section_string;
pub use find_act_ctx_section_string::{FindActCtxSectionStringA, FindActCtxSectionStringW};

mod lstrcat;
pub use lstrcat::{lstrcatA, lstrcatW};

mod lstrcmpi;
pub use lstrcmpi::{lstrcmpi, lstrcmpiA, lstrcmpiW};

mod lstrcpy;
pub use lstrcpy::{lstrcpy, lstrcpyA, lstrcpyW};

mod lstrcpyn;
pub use lstrcpyn::{lstrcpyn, lstrcpynA, lstrcpynW};

mod lstrlen;
pub use lstrlen::{lstrlenA, lstrlenW};

mod multi_byte_to_wide_char;
pub use multi_byte_to_wide_char::MultiByteToWideChar;

mod wide_char_to_multi_byte;
pub use wide_char_to_multi_byte::WideCharToMultiByte;

mod format_message;
pub use format_message::{FormatMessageA, FormatMessageW};

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "GetCalendarInfoA" => kernel32::GetCalendarInfoA(emu),
        "GetCalendarInfoW" => kernel32::GetCalendarInfoW(emu),
        "GetUserDefaultLangID" => kernel32::GetUserDefaultLangID(emu),
        "lstrcmpi" => kernel32::lstrcmpi(emu),
        "lstrcmpiA" => kernel32::lstrcmpiA(emu),
        "GetACP" => kernel32::GetACP(emu),
        "GetCPInfo" => kernel32::GetCPInfo(emu),
        "GetLocaleInfoA" => kernel32::GetLocaleInfoA(emu),
        "GetLocaleInfoW" => kernel32::GetLocaleInfoW(emu),
        "GetStringTypeW" => kernel32::GetStringTypeW(emu),
        "GetUserDefaultLCID" => kernel32::GetUserDefaultLCID(emu),
        "GlobalAddAtomA" => kernel32::GlobalAddAtomA(emu),
        "IsValidCodePage" => kernel32::IsValidCodePage(emu),
        "LCMapStringA" => kernel32::LCMapStringA(emu),
        "LCMapStringW" => kernel32::LCMapStringW(emu),
        "lstrcatA" => kernel32::lstrcatA(emu),
        "lstrcatW" => kernel32::lstrcatW(emu),
        "lstrcmpiW" => kernel32::lstrcmpiW(emu),
        "lstrcpy" => kernel32::lstrcpy(emu),
        "lstrcpyn" => kernel32::lstrcpyn(emu),
        "lstrcpynA" => kernel32::lstrcpynA(emu),
        "lstrcpynW" => kernel32::lstrcpynW(emu),
        "lstrcpyA" => kernel32::lstrcpyA(emu),
        "lstrcpyW" => kernel32::lstrcpyW(emu),
        "lstrlen" => kernel32::lstrlenA(emu),
        "lstrlenA" => kernel32::lstrlenA(emu),
        "lstrlenW" => kernel32::lstrlenW(emu),
        "MultiByteToWideChar" => kernel32::MultiByteToWideChar(emu),
        "WideCharToMultiByte" => kernel32::WideCharToMultiByte(emu),
        "lstrcmpA" => kernel32::lstrcmpA(emu),
        "lstrcmpW" => kernel32::lstrcmpW(emu),
        "GetStringTypeA" => kernel32::GetStringTypeA(emu),
        "GetOEMCP" => kernel32::GetOEMCP(emu),
        "GetUserDefaultUILanguage" => kernel32::GetUserDefaultUILanguage(emu),
        "SetLocaleInfoA" => kernel32::SetLocaleInfoA(emu),
        "SetLocaleInfoW" => kernel32::SetLocaleInfoW(emu),
        "GetLocaleInfoEx" => kernel32::GetLocaleInfoEx(emu),
        "GlobalAddAtomW" => kernel32::GlobalAddAtomW(emu),
        "GlobalDeleteAtom" => kernel32::GlobalDeleteAtom(emu),
        "GlobalFindAtomA" => kernel32::GlobalFindAtomA(emu),
        "GlobalFindAtomW" => kernel32::GlobalFindAtomW(emu),
        "GlobalGetAtomNameA" => kernel32::GlobalGetAtomNameA(emu),
        "GlobalGetAtomNameW" => kernel32::GlobalGetAtomNameW(emu),
        _ => return false,
    }
    true
}
