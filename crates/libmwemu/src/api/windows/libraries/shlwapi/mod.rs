use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::{paths, shell_strings};
mod path_file_exists;
mod path_find_suffix_array;
mod path_is_content_type;

pub use path_file_exists::{PathFileExistsA, PathFileExistsW};
pub use path_find_suffix_array::{PathFindSuffixArrayA, PathFindSuffixArrayW};
pub use path_is_content_type::{PathIsContentTypeA, PathIsContentTypeW};

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    if paths::dispatch(api, "shlwapi", emu) {
        return true;
    }
    dispatch_api!(api, {
        "StrChrA" | "StrChrW" | "StrChrIA" | "StrChrIW" | "StrStrA" | "StrStrW" | "StrStrIA" | "StrStrIW" | "StrPBrkA" | "StrPBrkW" | "StrSpnA" | "StrSpnW" | "StrCSpnA" | "StrCSpnW" | "StrCSpnIA" | "StrCSpnIW" => shell_strings::search(emu, api),
        "StrRChrA" | "StrRChrW" | "StrRChrIA" | "StrRChrIW" | "StrRStrIA" | "StrRStrIW" => shell_strings::reverse_search(emu, api),
        "StrCmpNA" | "StrCmpNW" | "StrCmpNIA" | "StrCmpNIW" => shell_strings::compare_bounded(emu, "shlwapi", api),
        "StrTrimA" | "StrTrimW" => shell_strings::trim(emu, api),
        "PathIsContentTypeA" => PathIsContentTypeA(emu),
        "PathIsContentTypeW" => PathIsContentTypeW(emu),
        "PathFindSuffixArrayW" => PathFindSuffixArrayW(emu),
        "PathFindSuffixArrayA" => PathFindSuffixArrayA(emu),
        "PathFileExistsA" => PathFileExistsA(emu),
        "PathFileExistsW" => PathFileExistsW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
