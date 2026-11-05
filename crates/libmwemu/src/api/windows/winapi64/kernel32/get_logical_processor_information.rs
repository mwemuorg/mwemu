use crate::emu;

pub fn GetLogicalProcessorInformation(emu: &mut emu::Emu) {
    let buffer_ptr = emu.regs().rcx;
    let length_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetLogicalProcessorInformation buf:0x{:x} len:0x{:x}",
        buffer_ptr,
        length_ptr
    );
    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 72);
    }
    emu.regs_mut().rax = 0;
    emu.set_last_error(122);
}
