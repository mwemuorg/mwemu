use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

mod url_download_to_file_a;
use url_download_to_file_a::URLDownloadToFileA;
mod url_download_to_file_w;
use url_download_to_file_w::URLDownloadToFileW;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "URLDownloadToFileA" => URLDownloadToFileA(emu),
        "URLDownloadToFileW" => URLDownloadToFileW(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }

    String::new()
}
