use crate::emu::Emu;
use crate::winapi::winapi64::wincrt;

mod mbstowcs;
pub use mbstowcs::mbstowcs;

mod wcstombs;
pub use wcstombs::wcstombs;

mod strncat;
pub use strncat::strncat;

mod wcsncat;
pub use wcsncat::wcsncat;

mod str_cmp_ca;
pub use str_cmp_ca::StrCmpCA;

mod _wcsnicmp;
pub use _wcsnicmp::_wcsnicmp;

mod _itow;
pub use _itow::_itow;

mod _wtol;
pub use _wtol::_wtol;

mod _mbstowcs_l;
pub use _mbstowcs_l::_mbstowcs_l;

mod _wcstombs_l;
pub use _wcstombs_l::_wcstombs_l;

mod ispunct;
pub use ispunct::ispunct;

mod isxdigit;
pub use isxdigit::isxdigit;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "strlen" => wincrt::strlen(emu),
        "strncmp" => wincrt::strncmp(emu),
        "strcmp" => wincrt::crt_strcmp(emu),
        "strncpy" => wincrt::crt_strncpy(emu),
        "strcpy" => wincrt::crt_strcpy(emu),
        "strcat" => wincrt::crt_strcat(emu),
        "strstr" => wincrt::crt_strstr(emu),
        "strchr" => wincrt::crt_strchr(emu),
        "strrchr" => wincrt::crt_strrchr(emu),
        "_stricmp" | "_strcmpi" => wincrt::crt_stricmp(emu),
        "_strnicmp" => wincrt::crt_strnicmp(emu),
        "strtol" => wincrt::crt_strtol(emu),
        "strtoul" => wincrt::crt_strtoul(emu),
        "atoi" => wincrt::crt_atoi(emu),
        "atol" => wincrt::crt_atoi(emu),
        "_itoa" => wincrt::crt_itoa(emu),
        "wcslen" => wincrt::crt_wcslen(emu),
        "wcscpy" => wincrt::crt_wcscpy(emu),
        "wcscat" => wincrt::crt_wcscat(emu),
        "wcscmp" => wincrt::crt_wcscmp(emu),
        "_wcsicmp" => wincrt::crt_wcsicmp(emu),
        "wcsncpy" => wincrt::crt_wcsncpy(emu),
        "_wtoi" => wincrt::crt_wtoi(emu),
        "towupper" => wincrt::crt_towupper(emu),
        "towlower" => wincrt::crt_towlower(emu),
        "toupper" => wincrt::crt_toupper(emu),
        "tolower" => wincrt::crt_tolower(emu),
        "isalpha" => wincrt::crt_isalpha(emu),
        "isdigit" => wincrt::crt_isdigit(emu),
        "isspace" => wincrt::crt_isspace(emu),
        "isalnum" => wincrt::crt_isalnum(emu),
        "isupper" => wincrt::crt_isupper(emu),
        "islower" => wincrt::crt_islower(emu),
        "strtok" | "strtok_s" => wincrt::crt_strtok(emu),
        "mbstowcs" => wincrt::mbstowcs(emu),
        "wcstombs" => wincrt::wcstombs(emu),
        "strncat" => wincrt::strncat(emu),
        "wcsncat" => wincrt::wcsncat(emu),
        "StrCmpCA" => wincrt::StrCmpCA(emu),
        "_wcsnicmp" => wincrt::_wcsnicmp(emu),
        "_itow" => wincrt::_itow(emu),
        "_wtol" => wincrt::_wtol(emu),
        "_mbstowcs_l" => wincrt::_mbstowcs_l(emu),
        "_wcstombs_l" => wincrt::_wcstombs_l(emu),
        "ispunct" => wincrt::ispunct(emu),
        "isxdigit" => wincrt::isxdigit(emu),
        _ => return false,
    }
    true
}
