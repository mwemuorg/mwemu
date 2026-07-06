use crate::emu;

pub fn FileTimeToLocalFileTime(emu: &mut emu::Emu) {
    let lpFileTime =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!FileTimeToLocalFileTime cannot read lpFileTime") as u64;
    let out_lpLocalFileTime = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!FileTimeToLocalFileTime cannot read out_lpLocalFileTime")
        as u64;

    let dwLowDateTime = emu
        .maps
        .read_dword(lpFileTime)
        .expect("kernel32!FileTimeToLocalFileTime cannot read dwLowDateTime");
    let dwHighDateTime = emu
        .maps
        .read_dword(lpFileTime + 4)
        .expect("kernel32!FileTimeToLocalFileTime cannot read dwHighDateTime");

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
