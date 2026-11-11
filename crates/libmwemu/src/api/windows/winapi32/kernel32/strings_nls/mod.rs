use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

#[path = "../set_environment_strings.rs"]
mod set_environment_strings;
pub use set_environment_strings::*;

#[path = "../lc_map_string.rs"]
mod lc_map_string;
pub use lc_map_string::*;

#[path = "../get_string_type.rs"]
mod get_string_type;
pub use get_string_type::*;

#[path = "../fold_string.rs"]
mod fold_string;
pub use fold_string::*;

#[path = "../find_act_ctx_section_string.rs"]
mod find_act_ctx_section_string;
pub use find_act_ctx_section_string::*;

#[path = "../compare_string.rs"]
mod compare_string;
pub use compare_string::*;

#[path = "../write_profile_string.rs"]
mod write_profile_string;
pub use write_profile_string::*;

#[path = "../write_private_profile_string.rs"]
mod write_private_profile_string;
pub use write_private_profile_string::*;

#[path = "../string_length_worker.rs"]
mod string_length_worker;
pub use string_length_worker::*;

#[path = "../string_copy_worker.rs"]
mod string_copy_worker;
pub use string_copy_worker::*;

#[path = "../lstrcmp.rs"]
mod lstrcmp;
pub use lstrcmp::*;

#[path = "../is_bad_string_ptr.rs"]
mod is_bad_string_ptr;
pub use is_bad_string_ptr::*;

#[path = "../get_profile_string.rs"]
mod get_profile_string;
pub use get_profile_string::*;

#[path = "../get_private_profile_string.rs"]
mod get_private_profile_string;
pub use get_private_profile_string::*;

#[path = "../get_locale_info.rs"]
mod get_locale_info;
pub use get_locale_info::*;

#[path = "../free_environment_strings.rs"]
mod free_environment_strings;
pub use free_environment_strings::*;

#[path = "../expand_environment_strings.rs"]
mod expand_environment_strings;
pub use expand_environment_strings::*;

#[path = "../get_environment_strings.rs"]
mod get_environment_strings;
pub use get_environment_strings::*;

#[path = "../lstrcat.rs"]
mod lstrcat;
pub use lstrcat::*;

#[path = "../lstrcmpi.rs"]
mod lstrcmpi;
pub use lstrcmpi::*;

#[path = "../lstrcpy.rs"]
mod lstrcpy;
pub use lstrcpy::*;

#[path = "../lstrlen.rs"]
mod lstrlen;
pub use lstrlen::*;

#[path = "../multi_byte_to_wide_char.rs"]
mod multi_byte_to_wide_char;
pub use multi_byte_to_wide_char::*;

#[path = "../wide_char_to_multi_byte.rs"]
mod wide_char_to_multi_byte;
pub use wide_char_to_multi_byte::*;

#[path = "../find_act_ctx_section_string_w_worker.rs"]
mod find_act_ctx_section_string_w_worker;
pub use find_act_ctx_section_string_w_worker::*;

#[path = "../load_string_base_w.rs"]
mod load_string_base_w;
pub use load_string_base_w::*;

#[path = "../open_sort_id_key.rs"]
mod open_sort_id_key;
pub use open_sort_id_key::*;

#[path = "../sort_change_case.rs"]
mod sort_change_case;
pub use sort_change_case::*;

#[path = "../sort_compare_string.rs"]
mod sort_compare_string;
pub use sort_compare_string::*;

#[path = "../sort_find_string.rs"]
mod sort_find_string;
pub use sort_find_string::*;

#[path = "../sort_get_hash_code.rs"]
mod sort_get_hash_code;
pub use sort_get_hash_code::*;

#[path = "../sort_is_defined_string.rs"]
mod sort_is_defined_string;
pub use sort_is_defined_string::*;

#[path = "../string_cb_cat_ex_w.rs"]
mod string_cb_cat_ex_w;
pub use string_cb_cat_ex_w::*;

#[path = "../string_cb_copy_ex_w.rs"]
mod string_cb_copy_ex_w;
pub use string_cb_copy_ex_w::*;

#[path = "../string_cb_copy_w.rs"]
mod string_cb_copy_w;
pub use string_cb_copy_w::*;

#[path = "../string_cch_cat_w.rs"]
mod string_cch_cat_w;
pub use string_cch_cat_w::*;

#[path = "../string_cch_copy_ex_w.rs"]
mod string_cch_copy_ex_w;
pub use string_cch_copy_ex_w::*;

#[path = "../string_cch_copy_nw.rs"]
mod string_cch_copy_nw;
pub use string_cch_copy_nw::*;

#[path = "../string_cch_copy_w.rs"]
mod string_cch_copy_w;
pub use string_cch_copy_w::*;

#[path = "../string_cch_length_w.rs"]
mod string_cch_length_w;
pub use string_cch_length_w::*;

#[path = "../string_copy_worker_w_2_0.rs"]
mod string_copy_worker_w_2_0;
pub use string_copy_worker_w_2_0::*;

#[path = "../string_ex_validate_dest_w.rs"]
mod string_ex_validate_dest_w;
pub use string_ex_validate_dest_w::*;

#[path = "../string_table.rs"]
mod string_table;
pub use string_table::*;

#[path = "../string_v_printf_worker_w.rs"]
mod string_v_printf_worker_w;
pub use string_v_printf_worker_w::*;

#[path = "../string_v_printf_worker_w_0.rs"]
mod string_v_printf_worker_w_0;

#[path = "../string_validate_dest_and_length_w.rs"]
mod string_validate_dest_and_length_w;
pub use string_validate_dest_and_length_w::*;

#[path = "../lstrcpyn.rs"]
mod lstrcpyn;
pub use lstrcpyn::*;

#[path = "../get_user_default_lcid.rs"]
mod get_user_default_lcid;
pub use get_user_default_lcid::*;

#[path = "../get_thread_locale.rs"]
mod get_thread_locale;
pub use get_thread_locale::*;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "lstrlenW" => kernel32::lstrlenW(emu),
        "lstrcpyW" => kernel32::lstrcpyW(emu),
        "lstrcpyn" => kernel32::lstrcpyn(emu),
        "lstrcatW" => kernel32::lstrcatW(emu),
        "lstrcmpiW" => kernel32::lstrcmpiW(emu),
        "GetLocaleInfoA" => kernel32::GetLocaleInfoA(emu),
        "GetLocaleInfoW" => kernel32::GetLocaleInfoW(emu),
        "GetUserDefaultLCID" => kernel32::GetUserDefaultLCID(emu),
        "lstrlenA" => kernel32::lstrlen(emu),
        "lstrcatA" => kernel32::lstrcat(emu),
        "GetACP" => kernel32::GetACP(emu),
        "GetCPInfo" => kernel32::GetCPInfo(emu),
        "GetOEMCP" => kernel32::GetOEMCP(emu),
        "GetStringTypeW" => kernel32::GetStringTypeW(emu),
        "GetUserDefaultLangID" => kernel32::GetUserDefaultLangID(emu),
        "GetUserDefaultUILanguage" => kernel32::GetUserDefaultUILanguage(emu),
        "GlobalAddAtomA" => kernel32::GlobalAddAtomA(emu),
        "IsValidCodePage" => kernel32::IsValidCodePage(emu),
        "IsValidLocale" => kernel32::IsValidLocale(emu),
        "LCMapStringW" => kernel32::LCMapStringW(emu),
        "lstrcat" => kernel32::lstrcat(emu),
        "lstrcpy" => kernel32::lstrcpy(emu),
        "lstrlen" => kernel32::lstrlen(emu),
        "MultiByteToWideChar" => kernel32::MultiByteToWideChar(emu),
        "WideCharToMultiByte" => kernel32::WideCharToMultiByte(emu),
        "AddAtomA" => kernel32::AddAtomA(emu),
        "AddAtomW" => kernel32::AddAtomW(emu),
        "BaseVerifyUnicodeString" => kernel32::BaseVerifyUnicodeString(emu),
        "Basep8BitStringToDynamicUnicodeString" => {
            kernel32::Basep8BitStringToDynamicUnicodeString(emu)
        }
        "BasepAnsiStringToDynamicUnicodeString" => {
            kernel32::BasepAnsiStringToDynamicUnicodeString(emu)
        }
        "DeleteAtom" => kernel32::DeleteAtom(emu),
        "EnumLanguageGroupLocalesA" => kernel32::EnumLanguageGroupLocalesA(emu),
        "EnumSystemCodePagesA" => kernel32::EnumSystemCodePagesA(emu),
        "EnumSystemGeoID" => kernel32::EnumSystemGeoID(emu),
        "EnumSystemGeoNames" => kernel32::EnumSystemGeoNames(emu),
        "EnumSystemLanguageGroupsA" => kernel32::EnumSystemLanguageGroupsA(emu),
        "EnumUILanguagesA" => kernel32::EnumUILanguagesA(emu),
        "EraseTape" => kernel32::EraseTape(emu),
        "FindAtomA" => kernel32::FindAtomA(emu),
        "FindAtomW" => kernel32::FindAtomW(emu),
        "FoldStringA" => kernel32::FoldStringA(emu),
        "GetAtomNameA" => kernel32::GetAtomNameA(emu),
        "GetAtomNameW" => kernel32::GetAtomNameW(emu),
        "GetCurrencyFormatA" => kernel32::GetCurrencyFormatA(emu),
        "GetCurrencyFormatW" => kernel32::GetCurrencyFormatW(emu),
        "GetCurrentJaEraIndex" => kernel32::GetCurrentJaEraIndex(emu),
        "GetDurationFormat" => kernel32::GetDurationFormat(emu),
        "GetGeoInfoA" => kernel32::GetGeoInfoA(emu),
        "GetGeoInfoEx" => kernel32::GetGeoInfoEx(emu),
        "GetGeoInfoW" => kernel32::GetGeoInfoW(emu),
        "GetJapaneseEraItems" => kernel32::GetJapaneseEraItems(emu),
        "GetKnownJapanEraCount" => kernel32::GetKnownJapanEraCount(emu),
        "GetMaxJapanEras" => kernel32::GetMaxJapanEras(emu),
        "GetMaxJapanYears" => kernel32::GetMaxJapanYears(emu),
        "GetNumberFormatA" => kernel32::GetNumberFormatA(emu),
        "GetNumberFormatW" => kernel32::GetNumberFormatW(emu),
        "GetUserDefaultGeoName" => kernel32::GetUserDefaultGeoName(emu),
        "GetUserGeoID" => kernel32::GetUserGeoID(emu),
        "GlobalAddAtomExA" => kernel32::GlobalAddAtomExA(emu),
        "GlobalAddAtomExW" => kernel32::GlobalAddAtomExW(emu),
        "GlobalAddAtomW" => kernel32::GlobalAddAtomW(emu),
        "GlobalDeleteAtom" => kernel32::GlobalDeleteAtom(emu),
        "GlobalFindAtomA" => kernel32::GlobalFindAtomA(emu),
        "GlobalFindAtomW" => kernel32::GlobalFindAtomW(emu),
        "GlobalGetAtomNameA" => kernel32::GlobalGetAtomNameA(emu),
        "GlobalGetAtomNameW" => kernel32::GlobalGetAtomNameW(emu),
        "InitAtomTable" => kernel32::InitAtomTable(emu),
        "LoadStringBaseW" => kernel32::LoadStringBaseW(emu),
        "lstrcmpA" => kernel32::lstrcmpA(emu),
        "lstrcmp" => kernel32::lstrcmpA(emu),
        "lstrcmpW" => kernel32::lstrcmpW(emu),
        "lstrcmpiA" => kernel32::lstrcmpiA(emu),
        "NotifyUILanguageChange" => kernel32::NotifyUILanguageChange(emu),
        "SetLocaleInfoA" => kernel32::SetLocaleInfoA(emu),
        "SetUserGeoID" => kernel32::SetUserGeoID(emu),
        "SetUserGeoName" => kernel32::SetUserGeoName(emu),
        "SortChangeCase" => kernel32::SortChangeCase(emu),
        "SortCompareString" => kernel32::SortCompareString(emu),
        "SortFindString" => kernel32::SortFindString(emu),
        "SortGetHashCode" => kernel32::SortGetHashCode(emu),
        "SortIsDefinedString" => kernel32::SortIsDefinedString(emu),
        "StringCbCatExW" => kernel32::StringCbCatExW(emu),
        "StringCbCopyExW" => kernel32::StringCbCopyExW(emu),
        "StringCbCopyW" => kernel32::StringCbCopyW(emu),
        "StringCchCatW" => kernel32::StringCchCatW(emu),
        "StringCchCopyExW" => kernel32::StringCchCopyExW(emu),
        "StringCchCopyNW" => kernel32::StringCchCopyNW(emu),
        "StringCchCopyW" => kernel32::StringCchCopyW(emu),
        "StringCchLengthW" => kernel32::StringCchLengthW(emu),
        "StringCopyWorkerA" => kernel32::StringCopyWorkerA(emu),
        "StringCopyWorkerW" => kernel32::StringCopyWorkerW(emu),
        "StringCopyWorkerW_2_0" => kernel32::StringCopyWorkerW_2_0(emu),
        "StringExValidateDestW" => kernel32::StringExValidateDestW(emu),
        "StringLengthWorkerA" => kernel32::StringLengthWorkerA(emu),
        "StringLengthWorkerW" => kernel32::StringLengthWorkerW(emu),
        "StringTable" => kernel32::StringTable(emu),
        "StringVPrintfWorkerW" => kernel32::StringVPrintfWorkerW(emu),
        "StringValidateDestAndLengthW" => kernel32::StringValidateDestAndLengthW(emu),
        "ConvertDefaultLocale" => kernel32::ConvertDefaultLocale(emu),
        "EnumLanguageGroupLocalesW" => kernel32::EnumLanguageGroupLocalesW(emu),
        "EnumSystemCodePagesW" => kernel32::EnumSystemCodePagesW(emu),
        "EnumSystemLanguageGroupsW" => kernel32::EnumSystemLanguageGroupsW(emu),
        "EnumSystemLocalesA" => kernel32::EnumSystemLocalesA(emu),
        "EnumSystemLocalesEx" => kernel32::EnumSystemLocalesEx(emu),
        "EnumSystemLocalesW" => kernel32::EnumSystemLocalesW(emu),
        "EnumUILanguagesW" => kernel32::EnumUILanguagesW(emu),
        "FindNLSString" => kernel32::FindNLSString(emu),
        "FindNLSStringEx" => kernel32::FindNLSStringEx(emu),
        "FindStringOrdinal" => kernel32::FindStringOrdinal(emu),
        "FoldStringW" => kernel32::FoldStringW(emu),
        "GetCPInfoExA" => kernel32::GetCPInfoExA(emu),
        "GetCPInfoExW" => kernel32::GetCPInfoExW(emu),
        "GetCurrencyFormatEx" => kernel32::GetCurrencyFormatEx(emu),
        "GetDurationFormatEx" => kernel32::GetDurationFormatEx(emu),
        "GetEraNameCountedString" => kernel32::GetEraNameCountedString(emu),
        "GetLocaleInfoEx" => kernel32::GetLocaleInfoEx(emu),
        "GetLogicalDriveStringsA" => kernel32::GetLogicalDriveStringsA(emu),
        "GetLogicalDriveStringsW" => kernel32::GetLogicalDriveStringsW(emu),
        "GetNLSVersion" => kernel32::GetNLSVersion(emu),
        "GetNLSVersionEx" => kernel32::GetNLSVersionEx(emu),
        "GetNumberFormatEx" => kernel32::GetNumberFormatEx(emu),
        "GetStringScripts" => kernel32::GetStringScripts(emu),
        "GetStringTypeA" => kernel32::GetStringTypeA(emu),
        "GetStringTypeExA" => kernel32::GetStringTypeExA(emu),
        "GetStringTypeExW" => kernel32::GetStringTypeExW(emu),
        "GetSystemDefaultLCID" => kernel32::GetSystemDefaultLCID(emu),
        "GetSystemDefaultLangID" => kernel32::GetSystemDefaultLangID(emu),
        "GetSystemDefaultLocaleName" => kernel32::GetSystemDefaultLocaleName(emu),
        "GetSystemDefaultUILanguage" => kernel32::GetSystemDefaultUILanguage(emu),
        "GetSystemPreferredUILanguages" => kernel32::GetSystemPreferredUILanguages(emu),
        "GetUILanguageInfo" => kernel32::GetUILanguageInfo(emu),
        "GetUserDefaultLocaleName" => kernel32::GetUserDefaultLocaleName(emu),
        "GetUserPreferredUILanguages" => kernel32::GetUserPreferredUILanguages(emu),
        "IdnToAscii" => kernel32::IdnToAscii(emu),
        "IdnToNameprepUnicode" => kernel32::IdnToNameprepUnicode(emu),
        "IdnToUnicode" => kernel32::IdnToUnicode(emu),
        "IsNLSDefinedString" => kernel32::IsNLSDefinedString(emu),
        "IsNormalizedString" => kernel32::IsNormalizedString(emu),
        "IsValidLanguageGroup" => kernel32::IsValidLanguageGroup(emu),
        "IsValidLocaleName" => kernel32::IsValidLocaleName(emu),
        "IsValidNLSVersion" => kernel32::IsValidNLSVersion(emu),
        "LCIDToLocaleName" => kernel32::LCIDToLocaleName(emu),
        "LCMapStringA" => kernel32::LCMapStringA(emu),
        "LCMapStringEx" => kernel32::LCMapStringEx(emu),
        "LoadStringBaseExW" => kernel32::LoadStringBaseExW(emu),
        "LocaleNameToLCID" => kernel32::LocaleNameToLCID(emu),
        "NlsGetCacheUpdateCount" => kernel32::NlsGetCacheUpdateCount(emu),
        "NlsUpdateLocale" => kernel32::NlsUpdateLocale(emu),
        "NlsUpdateSystemLocale" => kernel32::NlsUpdateSystemLocale(emu),
        "NormalizeString" => kernel32::NormalizeString(emu),
        "ResolveLocaleName" => kernel32::ResolveLocaleName(emu),
        "SetFileApisToANSI" => kernel32::SetFileApisToANSI(emu),
        "SetFileApisToOEM" => kernel32::SetFileApisToOEM(emu),
        "SetLocaleInfoW" => kernel32::SetLocaleInfoW(emu),
        "SortCloseHandle" => kernel32::SortCloseHandle(emu),
        "SortGetHandle" => kernel32::SortGetHandle(emu),
        "VerLanguageNameA" => kernel32::VerLanguageNameA(emu),
        "VerLanguageNameW" => kernel32::VerLanguageNameW(emu),
        "VerifyScripts" => kernel32::VerifyScripts(emu),
        "lstrcmpi" => kernel32::lstrcmpi(emu),
        "lstrcpyA" => kernel32::lstrcpyA_api(emu),
        "lstrcpynA" => kernel32::lstrcpynA_api(emu),
        "lstrcpynW" => kernel32::lstrcpynW_api(emu),
        "GetUserDefaultLangId" => kernel32::GetUserDefaultLangId(emu),
        "LStrCmpI" => kernel32::LStrCmpI(emu),
        "LStrCmpIW" => kernel32::LStrCmpIW(emu),
        "uaw_lstrcmpW" => kernel32::uaw_lstrcmpW_api(emu),
        "uaw_lstrcmpiW" => kernel32::uaw_lstrcmpiW_api(emu),
        "uaw_lstrlenW" => kernel32::uaw_lstrlenW_api(emu),
        "uaw_wcschr" => kernel32::uaw_wcschr_api(emu),
        "uaw_wcscpy" => kernel32::uaw_wcscpy_api(emu),
        "uaw_wcsicmp" => kernel32::uaw_wcsicmp_api(emu),
        "uaw_wcslen" => kernel32::uaw_wcslen_api(emu),
        "uaw_wcsrchr" => kernel32::uaw_wcsrchr_api(emu),
        _ => return false,
    }
    true
}
