use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub(super) fn URLDownloadToFileA(emu: &mut emu::Emu) {
    url_download_to_file(emu, "URLDownloadToFileA", false);
}

fn url_download_to_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let pcaller = emu.regs().rcx;
    let sz_url = emu.regs().rdx;
    let sz_filename = emu.regs().r8;
    let dw_reserved = emu.regs().r9;
    let lpfn_cb = WinApiAbi::arg(emu, 4);

    let url = strings::read_text(emu, sz_url, wide);
    let filename = strings::read_text(emu, sz_filename, wide);

    log_red!(emu, "urlmon!{api} url: {} filename: {}", url, filename);

    emu.regs_mut().rax = 0; // S_OK
}

pub(super) fn URLDownloadToFileW(emu: &mut emu::Emu) {
    url_download_to_file(emu, "URLDownloadToFileW", true);
}
