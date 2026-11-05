use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

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

fn URLDownloadToFileA(emu: &mut emu::Emu) {
    let pcaller = emu.regs().rcx;
    let sz_url = emu.regs().rdx;
    let sz_filename = emu.regs().r8;
    let dw_reserved = emu.regs().r9;
    let lpfn_cb = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let url = emu.maps.read_string(sz_url);
    let filename = emu.maps.read_string(sz_filename);

    log_red!(
        emu,
        "urlmon!URLDownloadToFileA url: {} filename: {}",
        url,
        filename
    );

    emu.regs_mut().rax = 0; // S_OK
}

fn URLDownloadToFileW(emu: &mut emu::Emu) {
    let pcaller = emu.regs().rcx;
    let sz_url = emu.regs().rdx;
    let sz_filename = emu.regs().r8;
    let dw_reserved = emu.regs().r9;
    let lpfn_cb = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let url = emu.maps.read_wide_string(sz_url);
    let filename = emu.maps.read_wide_string(sz_filename);

    log_red!(
        emu,
        "urlmon!URLDownloadToFileW url: {} filename: {}",
        url,
        filename
    );

    emu.regs_mut().rax = 0; // S_OK
}
