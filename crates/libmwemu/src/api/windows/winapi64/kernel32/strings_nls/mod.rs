use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

#[path = "../compare_string.rs"]
mod compare_string;
pub use compare_string::*;

#[path = "../find_act_ctx_section_string_w.rs"]
mod find_act_ctx_section_string_w;
pub use find_act_ctx_section_string_w::*;

#[path = "../lstrcat.rs"]
mod lstrcat;
pub use lstrcat::*;

#[path = "../lstrcmpi.rs"]
mod lstrcmpi;
pub use lstrcmpi::*;

#[path = "../lstrcpy.rs"]
mod lstrcpy;
pub use lstrcpy::*;

#[path = "../lstrcpyn.rs"]
mod lstrcpyn;
pub use lstrcpyn::*;

#[path = "../lstrlen.rs"]
mod lstrlen;
pub use lstrlen::*;

#[path = "../multi_byte_to_wide_char.rs"]
mod multi_byte_to_wide_char;
pub use multi_byte_to_wide_char::*;

#[path = "../wide_char_to_multi_byte.rs"]
mod wide_char_to_multi_byte;
pub use wide_char_to_multi_byte::*;

#[path = "../format_message.rs"]
mod format_message;
pub use format_message::*;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "GetACP" => kernel32::GetACP(emu),
        "GetCPInfo" => kernel32::GetCPInfo(emu),
        "GetLocaleInfoA" => kernel32::GetLocaleInfoA(emu),
        "GetLocaleInfoW" => kernel32::GetLocaleInfoW(emu),
        "GetStringTypeW" => kernel32::GetStringTypeW(emu),
        "GetUserDefaultLangId" => kernel32::GetUserDefaultLangId(emu),
        "GetUserDefaultLCID" => kernel32::GetUserDefaultLCID(emu),
        "GlobalAddAtomA" => kernel32::GlobalAddAtomA(emu),
        "IsValidCodePage" => kernel32::IsValidCodePage(emu),
        "LCMapStringW" => kernel32::LCMapStringW(emu),
        "lstrcatA" => kernel32::lstrcatA(emu),
        "lstrcatW" => kernel32::lstrcatW(emu),
        "LStrCmpI" => kernel32::LStrCmpI(emu),
        "lstrcmpiW" => kernel32::LStrCmpIW(emu),
        "LStrCmpIW" => kernel32::LStrCmpIW(emu),
        "lstrcpy" => kernel32::lstrcpy(emu),
        "lstrcpyn" => kernel32::lstrcpyn(emu),
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
        "SetLocaleInfoA" | "SetLocaleInfoW" => kernel32::SetLocaleInfoA(emu, api),
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
