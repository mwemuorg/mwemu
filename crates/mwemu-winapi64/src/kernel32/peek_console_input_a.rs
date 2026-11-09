use crate::emu;

pub fn PeekConsoleInputA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let buffer = emu.regs().rdx;
    let length = emu.regs().r8;
    let events_read_ptr = emu.regs().r9;

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
}
