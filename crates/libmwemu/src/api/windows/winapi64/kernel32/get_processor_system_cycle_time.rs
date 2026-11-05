use crate::emu;

pub fn GetProcessorSystemCycleTime(emu: &mut emu::Emu) {
    let group = emu.regs().rcx as u16;
    let buffer_ptr = emu.regs().rdx;
    let length_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetProcessorSystemCycleTime group:{} buf:0x{:x} len:0x{:x}",
        group,
        buffer_ptr,
        length_ptr
    );
    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 32);
    }
    emu.regs_mut().rax = 0;
    emu.set_last_error(122);
}
