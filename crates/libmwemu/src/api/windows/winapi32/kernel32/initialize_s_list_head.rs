use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn InitializeSListHead(emu: &mut emu::Emu) {
    let list_head = arg(emu, 0) as u64;

    log_red!(
        emu,
        "kernel32!InitializeSListHead listHead:0x{:x}",
        list_head
    );

    if list_head != 0 {
        emu.maps.write_qword(list_head, 0);
        emu.maps.write_qword(list_head + 8, 0);
    }
    stdcall_ret(emu, 1);
}
