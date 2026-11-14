use crate::emu;

pub fn EtwEventWriteTransfer(emu: &mut emu::Emu) {
    let reg_handle = emu.regs().rcx;
    let event_descriptor = emu.regs().rdx;
    let activity_id = emu.regs().r8;
    let _related_id = emu.regs().r9;
    let desc_str = if event_descriptor != 0 {
        emu.maps.read_string(event_descriptor)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!EventWriteTransfer handle:0x{:x} desc:'{}' activity:0x{:x}",
        reg_handle,
        desc_str,
        activity_id
    );
    emu.regs_mut().rax = 0;
}
