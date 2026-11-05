use crate::emu;

pub fn GetLogicalProcessorInformationEx(emu: &mut emu::Emu) {
    let relation = emu.regs().rcx as u32;
    let buffer_ptr = emu.regs().rdx;
    let length_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetLogicalProcessorInformationEx rel:{} buf:0x{:x} len:0x{:x}",
        relation,
        buffer_ptr,
        length_ptr
    );
    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 112);
    }
    emu.regs_mut().rax = 0;
    emu.set_last_error(122);
}
