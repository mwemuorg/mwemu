use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub(super) fn URLDownloadToFileA(emu: &mut emu::Emu) {
    url_download_to_file(emu, "URLDownloadToFileA", false);
}

fn url_download_to_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let pcaller = WinApiAbi::arg(emu, 0);
    let sz_url = WinApiAbi::arg(emu, 1);
    let sz_filename = WinApiAbi::arg(emu, 2);
    let dw_reserved = WinApiAbi::arg(emu, 3);
    let lpfn_cb = WinApiAbi::arg(emu, 4);

    let url = strings::read_text(emu, sz_url, wide);
    let filename = strings::read_text(emu, sz_filename, wide);

    log_red!(emu, "urlmon!{api} url: {} filename: {}", url, filename);

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 0; // S_OK
}

pub(super) fn URLDownloadToFileW(emu: &mut emu::Emu) {
    url_download_to_file(emu, "URLDownloadToFileW", true);
}
