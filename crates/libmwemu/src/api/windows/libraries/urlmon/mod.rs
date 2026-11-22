use crate::api::dispatch_api;
use crate::emu::Emu;
mod url_download_to_file;
pub use url_download_to_file::{URLDownloadToFileA, URLDownloadToFileW};

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "URLDownloadToFileA" => URLDownloadToFileA(emu),
        "URLDownloadToFileW" => URLDownloadToFileW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
