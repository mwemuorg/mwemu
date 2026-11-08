use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FatalAppExitW(emu: &mut emu::Emu) {
    let _action = arg(emu, 0) as u64;
    let msg_ptr = arg(emu, 1) as u64;

    let msg = if msg_ptr != 0 {
        emu.maps.read_wide_string(msg_ptr)
    } else {
        String::new()
    };

    log_red!(emu, "kernel32!FatalAppExitW '{}'", msg);

    emu.stop();
    stdcall_ret(emu, 2);
}
