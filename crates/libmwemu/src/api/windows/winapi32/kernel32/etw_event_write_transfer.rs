use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn EtwEventWriteTransfer(emu: &mut emu::Emu) {
    let reg_handle = WinApiAbi::arg(emu, 0);
    let event_descriptor = WinApiAbi::arg(emu, 1);
    let activity_id = WinApiAbi::arg(emu, 2) as u32;
    let related_id = WinApiAbi::arg(emu, 3) as u32;
    let desc_str = emu.maps.read_string(event_descriptor);
    log_red!(
        emu,
        "kernel32!EventWriteTransfer handle:0x{:x} desc:'{}' activity:0x{:x}",
        reg_handle,
        desc_str,
        activity_id
    );
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}
