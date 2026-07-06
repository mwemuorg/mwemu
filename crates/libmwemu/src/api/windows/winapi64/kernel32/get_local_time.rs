use std::time::{SystemTime, UNIX_EPOCH};

use crate::emu;

pub fn GetLocalTime(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("error getting the localtime");

    let seconds = duration.as_secs();
    let seconds_since_midnight = seconds % 86400;
    let hours = seconds_since_midnight / 3600;
    let minutes = (seconds_since_midnight % 3600) / 60;
    let seconds = seconds_since_midnight % 60;

    let mut buffer = [0u8; 8];
    buffer[0] = hours as u8;
    buffer[1] = minutes as u8;
    buffer[2] = seconds as u8;

    emu.maps.write_bytes(ptr, &buffer);

    log_red!(emu, "kernel32!GetLocalTime");
}
