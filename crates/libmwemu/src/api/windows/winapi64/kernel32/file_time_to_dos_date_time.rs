use crate::emu;

pub fn FileTimeToDosDateTime(emu: &mut emu::Emu) {
    let lpFileTime = emu.regs().rcx;
    let out_lpFatDate = emu.regs().rdx;
    let out_lpFatTime = emu.regs().r8;

    let dwLowDateTime = emu.maps.read_dword(lpFileTime).unwrap_or(0);
    let dwHighDateTime = emu.maps.read_dword(lpFileTime + 4).unwrap_or(0);

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

    emu.regs_mut().rax = 1;
}
