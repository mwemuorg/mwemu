use crate::emu;

pub fn EtwEventWriteTransfer(emu: &mut emu::Emu) {
    let reg_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let event_descriptor = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let activity_id = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let related_id = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
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
