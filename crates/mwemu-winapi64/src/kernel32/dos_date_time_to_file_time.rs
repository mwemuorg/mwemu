use crate::emu;

pub fn DosDateTimeToFileTime(emu: &mut emu::Emu) {
    let fat_date = emu.regs().rcx as u16;
    let fat_time = emu.regs().rdx as u16;
    let p_file_time = emu.regs().r8;

    log_red!(
        emu,
        "kernel32!DosDateTimeToFileTime date:0x{:x} time:0x{:x} pFileTime:0x{:x}",
        fat_date,
        fat_time,
        p_file_time
    );

    if p_file_time != 0 {
        let day = (fat_date & 0x1F) as u64;
        let month = ((fat_date >> 5) & 0xF) as u64;
        let year = ((fat_date >> 9) & 0x7F) as u64 + 1980;
        let sec = ((fat_time & 0x1F) as u64) * 2;
        let min = ((fat_time >> 5) & 0x3F) as u64;
        let hour = ((fat_time >> 11) & 0x1F) as u64;

        let days_since_1601 = (year - 1601) * 365 + (year - 1601) / 4 + month * 30 + day;
        let ticks = (days_since_1601 * 86400 + hour * 3600 + min * 60 + sec) * 10_000_000;
        emu.maps.write_dword(p_file_time, ticks as u32);
        emu.maps.write_dword(p_file_time + 4, (ticks >> 32) as u32);
    }
    emu.regs_mut().rax = 1;
}
