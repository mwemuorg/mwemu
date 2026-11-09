use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

mod get_file_version_info;
use get_file_version_info::GetFileVersionInfo;
mod get_file_version_info_size;
use get_file_version_info_size::GetFileVersionInfoSize;
mod ver_query_value;
use ver_query_value::VerQueryValue;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "GetFileVersionInfoSizeA" | "GetFileVersionInfoSizeW" => GetFileVersionInfoSize(emu),
        "GetFileVersionInfoA" | "GetFileVersionInfoW" => GetFileVersionInfo(emu),
        "VerQueryValueA" | "VerQueryValueW" => VerQueryValue(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}
