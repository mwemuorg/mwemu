use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn PeekConsoleInputA(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let buffer = arg(emu, 1) as u64;
    let length = arg(emu, 2) as u64;
    let events_read_ptr = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!PeekConsoleInputA handle:0x{:x} buf:0x{:x} len:{}",
        handle,
        buffer,
        length
    );

    // Write 0 events read
    if events_read_ptr != 0 {
        emu.maps.write_dword(events_read_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
