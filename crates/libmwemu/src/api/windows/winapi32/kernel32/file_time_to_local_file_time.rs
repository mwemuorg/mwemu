use crate::emu;

pub fn FileTimeToLocalFileTime(emu: &mut emu::Emu) {
    let lpFileTime = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let out_lpLocalFileTime = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

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

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
