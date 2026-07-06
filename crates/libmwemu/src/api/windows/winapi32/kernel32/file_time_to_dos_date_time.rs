use crate::emu;

pub fn FileTimeToDosDateTime(emu: &mut emu::Emu) {
    let lpFileTime =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!FileTimeToDosDateTime cannot read lpFileTime") as u64;
    let out_lpFatDate =
        emu.maps
            .read_dword(emu.regs().get_esp() + 4)
            .expect("kernel32!FileTimeToDosDateTime cannot read out_lpFatDate") as u64;
    let out_lpFatTime =
        emu.maps
            .read_dword(emu.regs().get_esp() + 8)
            .expect("kernel32!FileTimeToDosDateTime cannot read out_lpFatTime") as u64;

    let dwLowDateTime = emu
        .maps
        .read_dword(lpFileTime)
        .expect("kernel32!FileTimeToLocalFileTime cannot read dwLowDateTime");
    let dwHighDateTime = emu
        .maps
        .read_dword(lpFileTime + 4)
        .expect("kernel32!FileTimeToLocalFileTime cannot read dwHighDateTime");

    /*
    let ftSeconds = (dwLowDateTime as u64) | ((dwHighDateTime as u64) << 32);
    let posix_seconds = (ftSeconds / 10_000_000) - 11_644_473_600;
    let utc_dt = std::time::UNIX_EPOCH + std::time::Duration::from_secs(posix_seconds);
    let local_dt = DateTime::<chrono::Local>::from(utc_dt).with_timezone(&chrono::Local);
    let year = (local_dt.year() - 1980) as u16;
    let month = local_dt.month() as u16;
    let day = local_dt.day() as u16;
    let date = ((year << 9) | (month << 5) | day) as u16;
    let hour = local_dt.hour() as u16;
    let min = local_dt.minute() as u16;
    let sec = (local_dt.second() / 2) as u16;
    let time = ((hour << 11) | (min << 5) | sec) as u16;

    emu.maps.write_dword(out_lpFatDate, date as u32);
    emu.maps.write_dword(out_lpFatTime, time as u32);
    */

    emu.maps.write_dword(out_lpFatDate, 0);
    emu.maps.write_dword(out_lpFatTime, 0);

    log_red!(
        emu,
        "kernel32!FileTimeToDosDateTime {} {}",
        dwLowDateTime,
        dwHighDateTime
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
