use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;
//use crate::windows::constants;
//
mod path_file_exists;
use path_file_exists::PathFileExistsA;
use path_file_exists::PathFileExistsW;
mod path_find_suffix_array_a;
pub use path_find_suffix_array_a::PathFindSuffixArrayA;
mod path_is_content_type_w;
pub use path_is_content_type_w::PathIsContentTypeW;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "PathIsContentTypeW" => PathIsContentTypeW(emu),
        "PathFindSuffixArrayA" => PathFindSuffixArrayA(emu),
        "PathFileExistsA" => PathFileExistsA(emu),
        "PathFileExistsW" => PathFileExistsW(emu),

        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}
