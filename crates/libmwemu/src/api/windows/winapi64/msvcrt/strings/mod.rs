use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::msvcrt;

pub use crate::api::windows::libraries::msvcrt::strings::*;

mod str_cmp_ca;
pub use str_cmp_ca::StrCmpCA;

mod mbstowcs;
pub use mbstowcs::mbstowcs;

mod wcstombs;
pub use wcstombs::wcstombs;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "strtok" => msvcrt::strtok(emu),
        "atoi" => msvcrt::atoi(emu),
        "_itoa" | "_itow" => msvcrt::_itoa(emu, api),
        "strncmp" => msvcrt::strncmp(emu),
        "wcslen" => msvcrt::wcslen(emu),
        "wcscpy" => msvcrt::wcscpy(emu),
        "wcscat" => msvcrt::wcscat(emu),
        "wcscmp" => msvcrt::wcscmp(emu),
        "strcmp" => msvcrt::strcmp(emu),
        "strcpy" => msvcrt::strcpy(emu),
        "strncpy" => msvcrt::strncpy(emu),
        "strcat" => msvcrt::strcat(emu),
        "strncat" => msvcrt::strncat(emu),
        "strstr" => msvcrt::strstr(emu),
        "strchr" => msvcrt::strchr(emu),
        "strrchr" => msvcrt::strrchr(emu),
        "_stricmp" => msvcrt::_stricmp(emu),
        "_strnicmp" => msvcrt::_strnicmp(emu),
        "strlen" => msvcrt::strlen(emu),
        "strtol" => msvcrt::strtol(emu),
        "strtoul" => msvcrt::strtoul(emu),
        "atol" => msvcrt::atol(emu),
        "wcsncpy" => msvcrt::wcsncpy(emu),
        "wcsncat" => msvcrt::wcsncat(emu),
        "_wcsicmp" => msvcrt::_wcsicmp(emu),
        "_wcsnicmp" => msvcrt::_wcsnicmp(emu),
        "_wtoi" => msvcrt::_wtoi(emu),
        "_wtol" => msvcrt::_wtol(emu),
        "_strcmpi" => msvcrt::_strcmpi(emu),
        "strtok_s" => msvcrt::strtok_s(emu),
        "StrCmpCA" => msvcrt::StrCmpCA(emu),
        "mbstowcs" | "_mbstowcs_l" => msvcrt::mbstowcs(emu, api),
        "wcstombs" | "_wcstombs_l" => msvcrt::wcstombs(emu, api),
        "str" => msvcrt::str(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
