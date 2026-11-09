use crate::emu;

pub fn FileTimeToLocalFileTime(emu: &mut emu::Emu) {
    let lpFileTime = emu.regs().rcx;
    let out_lpLocalFileTime = emu.regs().rdx;

    let dwLowDateTime = emu.maps.read_dword(lpFileTime).unwrap_or(0);
    let dwHighDateTime = emu.maps.read_dword(lpFileTime + 4).unwrap_or(0);

    emu.maps.write_dword(out_lpLocalFileTime, dwLowDateTime);
    emu.maps
        .write_dword(out_lpLocalFileTime + 4, dwHighDateTime);

    log_red!(
        emu,
        "kernel32!FileTimeToLocalFileTime {} {}",
        dwLowDateTime,
        dwHighDateTime
    );

    emu.regs_mut().rax = 1;
}
