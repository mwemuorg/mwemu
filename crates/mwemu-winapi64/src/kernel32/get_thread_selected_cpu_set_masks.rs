use crate::emu;

pub fn GetThreadSelectedCpuSetMasks(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let masks_ptr = emu.regs().rdx;
    let mask_count = emu.regs().r8 as u32;
    let required_count_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetThreadSelectedCpuSetMasks h:0x{:x} count:{}",
        h_thread,
        mask_count
    );
    if required_count_ptr != 0 {
        emu.maps.write_dword(required_count_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
