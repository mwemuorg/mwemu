use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn LogUnexpectedCodepath(emu: &mut emu::Emu) {
    let msg_ptr = arg(emu, 0) as u64;
    let msg = emu.maps.read_string(msg_ptr);
    log_red!(emu, "kernel32!LogUnexpectedCodepath '{}'", msg);
    stdcall_ret(emu, 1);
}
