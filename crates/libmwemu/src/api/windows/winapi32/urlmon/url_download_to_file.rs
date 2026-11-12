use crate::emu;
use crate::winapi::common::strings;

pub(super) fn URLDownloadToFileA(emu: &mut emu::Emu) {
    url_download_to_file(emu, "URLDownloadToFileA", false);
}

fn url_download_to_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let pcaller = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sz_url = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let sz_filename = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_reserved = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let lpfn_cb = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

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
