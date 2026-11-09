use crate::emu::Emu;
use crate::winapi::winapi32::msvcrt;

#[path = "../atoi.rs"]
mod atoi;
pub use atoi::atoi;

#[path = "../_itoa.rs"]
mod _itoa;
pub use _itoa::_itoa;

#[path = "../strncmp.rs"]
mod strncmp;
pub use strncmp::strncmp;

#[path = "../toupper.rs"]
mod toupper;
pub use toupper::toupper;

#[path = "../tolower.rs"]
mod tolower;
pub use tolower::tolower;

#[path = "../wcslen.rs"]
mod wcslen;
pub use wcslen::wcslen;

#[path = "../wcscpy.rs"]
mod wcscpy;
pub use wcscpy::wcscpy;

#[path = "../wcscat.rs"]
mod wcscat;
pub use wcscat::wcscat;

#[path = "../wcscmp.rs"]
mod wcscmp;
pub use wcscmp::wcscmp;

#[path = "../strcmp.rs"]
mod strcmp;
pub use strcmp::strcmp;

#[path = "../strcpy.rs"]
mod strcpy;
pub use strcpy::strcpy;

#[path = "../strncpy.rs"]
mod strncpy;
pub use strncpy::strncpy;

#[path = "../strcat.rs"]
mod strcat;
pub use strcat::strcat;

#[path = "../strncat.rs"]
mod strncat;
pub use strncat::strncat;

#[path = "../strstr.rs"]
mod strstr;
pub use strstr::strstr;

#[path = "../strchr.rs"]
mod strchr;
pub use strchr::strchr;

#[path = "../strrchr.rs"]
mod strrchr;
pub use strrchr::strrchr;

#[path = "../_stricmp.rs"]
mod _stricmp;
pub use _stricmp::_stricmp;

#[path = "../_strnicmp.rs"]
mod _strnicmp;
pub use _strnicmp::_strnicmp;

#[path = "../strlen.rs"]
mod strlen;
pub use strlen::strlen;

#[path = "../strtol.rs"]
mod strtol;
pub use strtol::strtol;

#[path = "../strtoul.rs"]
mod strtoul;
pub use strtoul::strtoul;

#[path = "../atol.rs"]
mod atol;
pub use atol::atol;

#[path = "../wcsncpy.rs"]
mod wcsncpy;
pub use wcsncpy::wcsncpy;

#[path = "../wcsncat.rs"]
mod wcsncat;
pub use wcsncat::wcsncat;

#[path = "../_wcsicmp.rs"]
mod _wcsicmp;
pub use _wcsicmp::_wcsicmp;

#[path = "../_wcsnicmp.rs"]
mod _wcsnicmp;
pub use _wcsnicmp::_wcsnicmp;

#[path = "../_wtoi.rs"]
mod _wtoi;
pub use _wtoi::_wtoi;

#[path = "../_wtol.rs"]
mod _wtol;
pub use _wtol::_wtol;

#[path = "../towupper.rs"]
mod towupper;
pub use towupper::towupper;

#[path = "../towlower.rs"]
mod towlower;
pub use towlower::towlower;

#[path = "../mbstowcs.rs"]
mod mbstowcs;
pub use mbstowcs::mbstowcs;

#[path = "../wcstombs.rs"]
mod wcstombs;
pub use wcstombs::wcstombs;

#[path = "../isalpha.rs"]
mod isalpha;
pub use isalpha::isalpha;

#[path = "../_strcmpi.rs"]
mod _strcmpi;
pub use _strcmpi::_strcmpi;

#[path = "../str.rs"]
mod str;
pub use str::str;

#[path = "../strtok_s.rs"]
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
