use crate::emu;

pub(super) fn URLDownloadToFileW(emu: &mut emu::Emu) {
    let pcaller = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sz_url = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let sz_filename = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_reserved = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let lpfn_cb = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;

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
