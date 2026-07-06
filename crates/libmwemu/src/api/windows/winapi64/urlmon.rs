use crate::emu;
use crate::serialization;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "URLDownloadToFileA" => URLDownloadToFileA(emu),
        "URLDownloadToFileW" => URLDownloadToFileW(emu),
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

fn URLDownloadToFileA(emu: &mut emu::Emu) {
    let pcaller = emu.regs().rcx;
    let sz_url = emu.regs().rdx;
    let sz_filename = emu.regs().r8;
    let dw_reserved = emu.regs().r9;
    let lpfn_cb = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("urlmon!URLDownloadToFileA error reading lpfn_cb");

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
    let lpfn_cb = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("urlmon!URLDownloadToFileW error reading lpfn_cb");

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
