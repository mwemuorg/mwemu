use crate::emu;

pub fn GetSystemCpuSetInformation(emu: &mut emu::Emu) {
    let information_ptr = emu.regs().rcx;
    let buffer_length = emu.regs().rdx as u32;
    let returned_length_ptr = emu.regs().r8;
    let h_process = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetSystemCpuSetInformation buf:0x{:x} len:{} ret:0x{:x} h:0x{:x}",
        information_ptr,
        buffer_length,
        returned_length_ptr,
        h_process
    );
    if returned_length_ptr != 0 {
        emu.maps.write_dword(returned_length_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
