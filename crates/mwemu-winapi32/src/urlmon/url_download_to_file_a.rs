use crate::emu;

pub(super) fn URLDownloadToFileA(emu: &mut emu::Emu) {
    let pcaller = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sz_url = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let sz_filename = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_reserved = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let lpfn_cb = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

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
