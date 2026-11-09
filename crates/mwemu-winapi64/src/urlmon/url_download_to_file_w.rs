use crate::emu;

pub(super) fn URLDownloadToFileW(emu: &mut emu::Emu) {
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
