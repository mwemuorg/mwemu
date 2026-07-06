use crate::emu;
use crate::serialization;
use crate::winapi::winapi32::kernel32;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
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
    let pcaller = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("urlmon!URLDownloadToFileA error reading pcaller") as u64;
    let sz_url = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("urlmon!URLDownloadToFileA error reading sz_url") as u64;
    let sz_filename = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("urlmon!URLDownloadToFileA error reading sz_filename") as u64;
    let dw_reserved = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("urlmon!URLDownloadToFileA error reading dw_reserved") as u64;
    let lpfn_cb = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("urlmon!URLDownloadToFileA error reading lpfn_cb") as u64;

    let url = emu.maps.read_string(sz_url);
    let filename = emu.maps.read_string(sz_filename);

    log_red!(
        emu,
        "urlmon!URLDownloadToFileA url: {} filename: {}",
        url,
        filename
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 0; // S_OK
}

fn URLDownloadToFileW(emu: &mut emu::Emu) {
    let pcaller = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("urlmon!URLDownloadToFileW error reading pcaller") as u64;
    let sz_url = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("urlmon!URLDownloadToFileW error reading sz_url") as u64;
    let sz_filename = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("urlmon!URLDownloadToFileW error reading sz_filename") as u64;
    let dw_reserved = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("urlmon!URLDownloadToFileW error reading dw_reserved") as u64;
    let lpfn_cb = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("urlmon!URLDownloadToFileW error reading lpfn_cb") as u64;

    let url = emu.maps.read_wide_string(sz_url);
    let filename = emu.maps.read_wide_string(sz_filename);

    log_red!(
        emu,
        "urlmon!URLDownloadToFileW url: {} filename: {}",
        url,
        filename
    );

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 0; // S_OK
}
