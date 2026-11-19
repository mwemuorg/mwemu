use crate::api::dispatch_api;
use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod url_download_to_file;
use url_download_to_file::URLDownloadToFileA;
use url_download_to_file::URLDownloadToFileW;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    dispatch_api!(api, {
        "URLDownloadToFileA" => URLDownloadToFileA(emu),
        "URLDownloadToFileW" => URLDownloadToFileW(emu),
    },
        handled(()) => String::new(),
        fallback => fallback::unimplemented(addr, api, emu),
    )
}
