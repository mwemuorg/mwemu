use crate::emu;
use crate::winapi::helper;

pub fn GetNumberOfConsoleInputEvents(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let num_events_ptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!GetNumberOfConsoleInputEvents handle:0x{:x} out:0x{:x}",
        handle,
        num_events_ptr
    );

    // Write 0 events to the output pointer
    if num_events_ptr != 0 {
        emu.maps.write_dword(num_events_ptr, 0);
    }

    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
