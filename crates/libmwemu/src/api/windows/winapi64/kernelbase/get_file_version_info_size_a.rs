use crate::emu;

pub(super) fn GetFileVersionInfoSizeA(emu: &mut emu::Emu) {
    let lptstr_filename = emu.regs().rcx;
    let lpdw_handle = emu.regs().rdx as usize;

    let filename = if lptstr_filename > 0 {
        emu.maps.read_string(lptstr_filename)
    } else {
        "unknown".to_string()
    };

    log_red!(
        emu,
        "** {} kernelbase!GetFileVersionInfoSizeA filename: {} lpdw_handle: 0x{:x}",
        emu.pos,
        filename,
        lpdw_handle
    );

    if filename == "comctl32.dll" {
        let dll_path = format!("{}/comctl32.dll", emu.cfg.maps_folder);
        match std::fs::metadata(dll_path) {
            Ok(metadata) => {
                emu.regs_mut().rax = metadata.len();
            }
            Err(e) => {
                log::warn!("GetFileVersionInfoSizeA: cannot stat comctl32.dll: {}", e);
                emu.regs_mut().rax = 0;
            }
        }
    } else {
        log::warn!("GetFileVersionInfoSizeA: unsupported file {}", filename);
        emu.regs_mut().rax = 0;
    }
}
