use crate::emu::Emu;
use crate::winapi::winapi32::msvcrt;

mod atoi;
pub use atoi::atoi;

mod _itoa;
pub use _itoa::_itoa;

mod strncmp;
pub use strncmp::strncmp;

mod toupper;
pub use toupper::toupper;

mod tolower;
pub use tolower::tolower;

mod wcslen;
pub use wcslen::wcslen;

mod wcscpy;
pub use wcscpy::wcscpy;

mod wcscat;
pub use wcscat::wcscat;

mod wcscmp;
pub use wcscmp::wcscmp;

mod strcmp;
pub use strcmp::strcmp;

mod strcpy;
pub use strcpy::strcpy;

mod strncpy;
pub use strncpy::strncpy;

mod strcat;
pub use strcat::strcat;

mod strncat;
pub use strncat::strncat;

mod strstr;
pub use strstr::strstr;

mod strchr;
pub use strchr::strchr;

mod strrchr;
pub use strrchr::strrchr;

mod _stricmp;
pub use _stricmp::_stricmp;

mod _strnicmp;
pub use _strnicmp::_strnicmp;

mod strlen;
pub use strlen::strlen;

mod strtol;
pub use strtol::strtol;

mod strtoul;
pub use strtoul::strtoul;

mod atol;
pub use atol::atol;

mod wcsncpy;
pub use wcsncpy::wcsncpy;

mod wcsncat;
pub use wcsncat::wcsncat;

mod _wcsicmp;
pub use _wcsicmp::_wcsicmp;

mod _wcsnicmp;
pub use _wcsnicmp::_wcsnicmp;

mod _wtoi;
pub use _wtoi::_wtoi;

mod _wtol;
pub use _wtol::_wtol;

mod towupper;
pub use towupper::towupper;

mod towlower;
pub use towlower::towlower;

mod mbstowcs;
pub use mbstowcs::mbstowcs;

mod wcstombs;
pub use wcstombs::wcstombs;

mod isalpha;
pub use isalpha::isalpha;

mod _strcmpi;
pub use _strcmpi::_strcmpi;

mod str;
pub use str::str;

mod strtok_s;
pub use strtok_s::strtok_s;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "StrCmpCA" => msvcrt::StrCmpCA(emu),
        "strtok" => msvcrt::strtok(emu),
        "atoi" => msvcrt::atoi(emu),
        "_itoa" | "_itow" => msvcrt::_itoa(emu, api),
        "strncmp" => msvcrt::strncmp(emu),
        "toupper" => msvcrt::toupper(emu),
        "tolower" => msvcrt::tolower(emu),
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
        "towupper" => msvcrt::towupper(emu),
        "towlower" => msvcrt::towlower(emu),
        "mbstowcs" | "_mbstowcs_l" => msvcrt::mbstowcs(emu, api),
        "wcstombs" | "_wcstombs_l" => msvcrt::wcstombs(emu, api),
        "isalpha" | "isdigit" | "isspace" | "isalnum" | "ispunct" | "isupper" | "islower"
        | "isxdigit" => msvcrt::isalpha(emu, api),
        "_strcmpi" => msvcrt::_strcmpi(emu),
        "str" => msvcrt::str(emu),
        "strtok_s" => msvcrt::strtok_s(emu),
        _ => return false,
    }
    true
}
