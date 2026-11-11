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
        let dll_path = emu.cfg.get_maps_folder("comctl32.dll");
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

pub fn GetFileVersionInfoSizeW(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernelbase!GetFileVersionInfoSizeW '{}'  p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
