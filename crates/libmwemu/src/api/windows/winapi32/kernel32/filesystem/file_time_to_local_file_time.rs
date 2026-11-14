use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FileTimeToLocalFileTime(emu: &mut emu::Emu) {
    let lpFileTime = WinApiAbi::arg(emu, 0);
    let out_lpLocalFileTime = WinApiAbi::arg(emu, 1);

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
